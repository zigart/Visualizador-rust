use chrono::{DateTime, Utc};

use dominio::{MovimientoRecorrido, Operacion, RepositorioRecorrido};

pub struct RepositorioRecorridoEnTransaccion {
    id_recorrido_ya_existe: bool,
    movimientos_usuario: Vec<MovimientoRecorrido>,
}

impl RepositorioRecorridoEnTransaccion {
    pub fn new(
        id_recorrido_ya_existe: bool,
        movimientos_usuario: Vec<MovimientoRecorrido>,
    ) -> Self {
        Self {
            id_recorrido_ya_existe,
            movimientos_usuario,
        }
    }
}

impl RepositorioRecorrido for RepositorioRecorridoEnTransaccion {
    fn existe_id_recorrido(&self, id_recorrido: u64) -> bool {
        self.id_recorrido_ya_existe
            || self
                .movimientos_usuario
                .iter()
                .any(|movimiento| movimiento.id_recorrido == id_recorrido)
    }

    fn retiro_activo_usuario(
        &self,
        id_usuario: u64,
        fechahora: DateTime<Utc>,
    ) -> Option<MovimientoRecorrido> {
        match self.ultimo_movimiento_antes(id_usuario, fechahora) {
            Some(movimiento) if movimiento.operacion == Operacion::Retiro => Some(movimiento),
            _ => None,
        }
    }

    fn ultimo_movimiento_antes(
        &self,
        id_usuario: u64,
        fechahora: DateTime<Utc>,
    ) -> Option<MovimientoRecorrido> {
        self.movimientos_usuario
            .iter()
            .filter(|movimiento| {
                movimiento.id_usuario == id_usuario && movimiento.fechahora <= fechahora
            })
            .max_by_key(|movimiento| (movimiento.fechahora, movimiento.id_recorrido))
            .cloned()
    }

    fn siguiente_movimiento_despues(
        &self,
        id_usuario: u64,
        fechahora: DateTime<Utc>,
    ) -> Option<MovimientoRecorrido> {
        self.movimientos_usuario
            .iter()
            .filter(|movimiento| {
                movimiento.id_usuario == id_usuario && movimiento.fechahora > fechahora
            })
            .min_by_key(|movimiento| (movimiento.fechahora, movimiento.id_recorrido))
            .cloned()
    }
}
