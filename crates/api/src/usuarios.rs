use std::{collections::BTreeMap, future::Future, pin::Pin, sync::Arc};

#[cfg(test)]
use std::sync::Mutex;

use adaptadores::PersistenciaMovimientos;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use dominio::{MovimientoRecorrido, Operacion};
use serde::Serialize;
use tracing::error;

#[derive(Clone)]
pub struct UsuariosState {
    pub recorridos: Arc<dyn ConsultaRepositorioRecorrido>,
}

pub trait ConsultaRepositorioRecorrido: Send + Sync {
    fn listar_movimientos_por_usuario(
        &self,
        id_usuario: u64,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<MovimientoRecorrido>>> + Send + '_>>;
}

impl ConsultaRepositorioRecorrido for PersistenciaMovimientos {
    fn listar_movimientos_por_usuario(
        &self,
        id_usuario: u64,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<MovimientoRecorrido>>> + Send + '_>> {
        self.consulta_listar_movimientos(id_usuario)
    }
}

#[derive(Debug, Default)]
#[cfg(test)]
pub struct RepositorioRecorridoMemoria {
    movimientos: Mutex<Vec<MovimientoRecorrido>>,
}

#[cfg(test)]
impl RepositorioRecorridoMemoria {
    pub fn registrar_movimiento(&self, movimiento: MovimientoRecorrido) {
        self.movimientos
            .lock()
            .expect("recorridos bloqueado")
            .push(movimiento);
    }
}

#[cfg(test)]
impl ConsultaRepositorioRecorrido for RepositorioRecorridoMemoria {
    fn listar_movimientos_por_usuario(
        &self,
        id_usuario: u64,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<MovimientoRecorrido>>> + Send + '_>> {
        Box::pin(async move {
            Ok(self
                .movimientos
                .lock()
                .expect("recorridos bloqueado")
                .iter()
                .filter(|movimiento| movimiento.id_usuario == id_usuario)
                .cloned()
                .collect())
        })
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViajesResponse {
    pub viajes: Vec<Viaje>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Viaje {
    pub id_recorrido: u64,
    pub fecha_hora_retiro: String,
    pub id_estacion_retiro: Option<u64>,
    pub fecha_hora_devolucion: Option<String>,
    pub id_estacion_devolucion: Option<u64>,
}

#[derive(Debug, Clone)]
struct ViajeBuilder {
    retiro: Option<MovimientoRecorrido>,
    devolucion: Option<MovimientoRecorrido>,
}

pub fn router(state: UsuariosState) -> Router {
    Router::new()
        .route("/usuarios/:id_usuario", get(historial_usuario))
        .with_state(state)
}

async fn historial_usuario(
    State(state): State<UsuariosState>,
    Path(id_usuario): Path<u64>,
) -> Result<Json<ViajesResponse>, StatusCode> {
    let movimientos = state
        .recorridos
        .listar_movimientos_por_usuario(id_usuario)
        .await
        .map_err(|err| {
            error!(error = %err, id_usuario, "error consultando historial de usuario");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if movimientos.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    let viajes = armar_viajes(movimientos);

    if viajes.is_empty() {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json(ViajesResponse { viajes }))
    }
}

pub fn armar_viajes(movimientos: Vec<MovimientoRecorrido>) -> Vec<Viaje> {
    let mut agrupados: BTreeMap<u64, ViajeBuilder> = BTreeMap::new();

    for movimiento in movimientos {
        let entry = agrupados
            .entry(movimiento.id_recorrido)
            .or_insert(ViajeBuilder {
                retiro: None,
                devolucion: None,
            });

        match movimiento.operacion {
            Operacion::Retiro => entry.retiro = Some(movimiento),
            Operacion::Devolucion => entry.devolucion = Some(movimiento),
        }
    }

    let mut viajes = agrupados
        .into_iter()
        .filter_map(|(id_recorrido, viaje)| {
            let retiro = viaje.retiro?;
            Some(Viaje {
                id_recorrido,
                fecha_hora_retiro: retiro.fechahora.to_rfc3339(),
                id_estacion_retiro: Some(retiro.id_estacion),
                fecha_hora_devolucion: viaje
                    .devolucion
                    .as_ref()
                    .map(|movimiento| movimiento.fechahora.to_rfc3339()),
                id_estacion_devolucion: viaje.devolucion.map(|movimiento| movimiento.id_estacion),
            })
        })
        .collect::<Vec<_>>();

    viajes.sort_by(
        |a, b| match (&a.fecha_hora_devolucion, &b.fecha_hora_devolucion) {
            (Some(a_fecha), Some(b_fecha)) => a_fecha
                .cmp(b_fecha)
                .then_with(|| a.id_recorrido.cmp(&b.id_recorrido)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.id_recorrido.cmp(&b.id_recorrido),
        },
    );

    viajes
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use chrono::{TimeZone, Utc};
    use dominio::{MovimientoRecorrido, Operacion};
    use tower::ServiceExt;

    use super::{router, RepositorioRecorridoMemoria, UsuariosState};

    #[tokio::test]
    async fn get_usuario_retorna_viajes_ordenados_y_campos_camel_case() {
        let repo = Arc::new(RepositorioRecorridoMemoria::default());
        repo.registrar_movimiento(movimiento(2, 1, 20, Operacion::Retiro, 1));
        repo.registrar_movimiento(movimiento(2, 1, 21, Operacion::Devolucion, 5));
        repo.registrar_movimiento(movimiento(1, 1, 10, Operacion::Retiro, 1));
        repo.registrar_movimiento(movimiento(1, 1, 11, Operacion::Devolucion, 3));

        let app = router(UsuariosState { recorridos: repo });
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/usuarios/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["viajes"][0]["idRecorrido"], 1);
        assert_eq!(json["viajes"][0]["idEstacionRetiro"], 10);
        assert_eq!(json["viajes"][0]["idEstacionDevolucion"], 11);
        assert!(json["viajes"][0].get("fechaHoraRetiro").is_some());
        assert!(json["viajes"][0].get("fechaHoraDevolucion").is_some());
        assert_eq!(json["viajes"][1]["idRecorrido"], 2);
    }

    #[tokio::test]
    async fn get_usuario_incluye_viaje_en_curso_con_devolucion_null() {
        let repo = Arc::new(RepositorioRecorridoMemoria::default());
        repo.registrar_movimiento(movimiento(7, 1, 70, Operacion::Retiro, 1));

        let app = router(UsuariosState { recorridos: repo });
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/usuarios/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["viajes"][0]["idRecorrido"], 7);
        assert_eq!(json["viajes"][0]["idEstacionRetiro"], 70);
        assert!(json["viajes"][0]["fechaHoraDevolucion"].is_null());
        assert!(json["viajes"][0]["idEstacionDevolucion"].is_null());
    }

    #[tokio::test]
    async fn get_usuario_inexistente_retorna_404() {
        let app = router(UsuariosState {
            recorridos: Arc::new(RepositorioRecorridoMemoria::default()),
        });
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/usuarios/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    fn movimiento(
        id_recorrido: u64,
        id_usuario: u64,
        id_estacion: u64,
        operacion: Operacion,
        dia: u32,
    ) -> MovimientoRecorrido {
        MovimientoRecorrido {
            id_recorrido,
            id_usuario,
            id_estacion,
            operacion,
            fechahora: Utc.with_ymd_and_hms(2026, 7, dia, 12, 0, 0).unwrap(),
        }
    }
}
