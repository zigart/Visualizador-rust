use std::{future::Future, pin::Pin};

use dominio::{ErrorDominio, EstadoBicicletas, MovimientoRecorrido, SistemaBicicletas};
use sqlx::{PgConnection, PgPool, Postgres, Transaction};

use super::{estado, recorrido, repo_sync::RepositorioRecorridoEnTransaccion};

#[derive(Debug, Clone)]
pub struct PersistenciaMovimientos {
    pool: PgPool,
}

impl PersistenciaMovimientos {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn leer_estado_actual(&self) -> anyhow::Result<EstadoBicicletas> {
        estado::leer_estado(&self.pool).await
    }

    pub async fn listar_movimientos_por_usuario(
        &self,
        id_usuario: u64,
    ) -> anyhow::Result<Vec<MovimientoRecorrido>> {
        recorrido::listar_movimientos_usuario(&self.pool, id_usuario).await
    }

    pub async fn persistir_movimiento(
        &self,
        movimiento: &MovimientoRecorrido,
    ) -> Result<EstadoBicicletas, ErrorPersistencia> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|error| ErrorPersistencia::base_datos(error.into()))?;

        match self
            .persistir_movimiento_en_tx(&mut tx, movimiento)
            .await
        {
            Ok(estado) => {
                tx.commit()
                    .await
                    .map_err(|error| ErrorPersistencia::base_datos(error.into()))?;
                Ok(estado)
            }
            Err(error) => {
                let _ = tx.rollback().await;
                Err(error)
            }
        }
    }

    pub fn consulta_listar_movimientos(
        &self,
        id_usuario: u64,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<MovimientoRecorrido>>> + Send + '_>> {
        let persistencia = self.clone();
        Box::pin(async move {
            persistencia
                .listar_movimientos_por_usuario(id_usuario)
                .await
        })
    }

    async fn persistir_movimiento_en_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movimiento: &MovimientoRecorrido,
    ) -> Result<EstadoBicicletas, ErrorPersistencia> {
        let conn: &mut PgConnection = &mut *tx;

        let estado_actual = estado::leer_estado_conn(conn)
            .await
            .map_err(ErrorPersistencia::base_datos)?;

        let existe_id = recorrido::existe_id_recorrido(conn, movimiento.id_recorrido)
            .await
            .map_err(ErrorPersistencia::base_datos)?;
        let movimientos_usuario =
            recorrido::listar_movimientos_usuario_conn(conn, movimiento.id_usuario)
                .await
                .map_err(ErrorPersistencia::base_datos)?;

        let repositorio =
            RepositorioRecorridoEnTransaccion::new(existe_id, movimientos_usuario);
        let nuevo_estado = SistemaBicicletas::new(repositorio)
            .procesar(movimiento, estado_actual)
            .map_err(ErrorPersistencia::Dominio)?;

        recorrido::insertar_movimiento(conn, movimiento)
            .await
            .map_err(ErrorPersistencia::base_datos)?;
        estado::actualizar_estado(conn, nuevo_estado)
            .await
            .map_err(ErrorPersistencia::base_datos)?;

        Ok(nuevo_estado)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorPersistencia {
    #[error(transparent)]
    Dominio(#[from] ErrorDominio),
    #[error("error de base de datos: {0}")]
    BaseDatos(#[from] anyhow::Error),
}

impl ErrorPersistencia {
    fn base_datos(error: anyhow::Error) -> Self {
        Self::BaseDatos(error)
    }

    pub fn es_error_dominio(&self) -> bool {
        matches!(self, Self::Dominio(_))
    }
}
