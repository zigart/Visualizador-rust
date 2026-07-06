use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use chrono::Utc;
use dominio::{EstadoBicicletas, MovimientoRecorrido};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tracing::{debug, warn};

#[derive(Clone)]
pub struct DashboardState {
    pub vista: Arc<dyn VistaEstadoBicicletas>,
    pub broadcaster: broadcast::Sender<EstadoDashboard>,
}

pub trait VistaEstadoBicicletas: Send + Sync {
    fn estado_actual(&self) -> EstadoDashboard;

    fn establecer_estado(&self, estado: EstadoBicicletas);
}

#[derive(Debug)]
pub struct VistaEstadoMemoria {
    estado: Mutex<EstadoBicicletas>,
}

impl VistaEstadoMemoria {
    pub fn new(estado: EstadoBicicletas) -> Self {
        Self {
            estado: Mutex::new(estado),
        }
    }

    pub fn actualizar_estado(&self, estado: EstadoBicicletas) {
        *self.estado.lock().expect("estado dashboard bloqueado") = estado;
    }
}

impl VistaEstadoBicicletas for VistaEstadoMemoria {
    fn estado_actual(&self) -> EstadoDashboard {
        let estado = self.estado.lock().expect("estado dashboard bloqueado");
        EstadoDashboard::from_estado(*estado, None)
    }

    fn establecer_estado(&self, estado: EstadoBicicletas) {
        self.actualizar_estado(estado);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EstadoDashboard {
    pub bicicletas_disponibles: u64,
    pub bicicletas_en_uso: u64,
    pub actualizado_en: String,
    pub estado_conexion: String,
    pub movimiento: Option<MovimientoDashboard>,
}

impl EstadoDashboard {
    pub fn from_estado(estado: EstadoBicicletas, movimiento: Option<MovimientoDashboard>) -> Self {
        Self {
            bicicletas_disponibles: estado.bicicletas_disponibles(),
            bicicletas_en_uso: estado.en_uso,
            actualizado_en: Utc::now().to_rfc3339(),
            estado_conexion: "conectado".to_string(),
            movimiento,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MovimientoDashboard {
    pub id_recorrido: u64,
    pub id_usuario: u64,
    pub operacion: String,
    pub fechahora: String,
}

impl From<MovimientoRecorrido> for MovimientoDashboard {
    fn from(value: MovimientoRecorrido) -> Self {
        Self {
            id_recorrido: value.id_recorrido,
            id_usuario: value.id_usuario,
            operacion: operacion_label(value.operacion).to_string(),
            fechahora: value.fechahora.to_rfc3339(),
        }
    }
}

pub fn router(state: DashboardState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_html))
        .route("/dashboard/styles.css", get(dashboard_css))
        .route("/dashboard/app.js", get(dashboard_js))
        .route("/ws", get(websocket_handler))
        .with_state(state)
}

pub async fn publicar_estado_dashboard(
    state: &DashboardState,
    movimiento: MovimientoRecorrido,
    estado: EstadoBicicletas,
) -> Result<()> {
    state.vista.establecer_estado(estado);

    let payload = EstadoDashboard::from_estado(estado, Some(MovimientoDashboard::from(movimiento)));

    match state.broadcaster.send(payload) {
        Ok(clientes) => debug!(clientes, "estado dashboard emitido"),
        Err(error) => warn!(error = %error, "no hay clientes WebSocket para recibir dashboard"),
    }

    Ok(())
}

async fn dashboard_html() -> Html<&'static str> {
    Html(include_str!("../../../static/dashboard/index.html"))
}

async fn dashboard_css() -> Response {
    (
        [("content-type", "text/css; charset=utf-8")],
        include_str!("../../../static/dashboard/styles.css"),
    )
        .into_response()
}

async fn dashboard_js() -> Response {
    (
        [("content-type", "application/javascript; charset=utf-8")],
        include_str!("../../../static/dashboard/app.js"),
    )
        .into_response()
}

async fn websocket_handler(
    State(state): State<DashboardState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket_session(socket, state))
}

async fn websocket_session(mut socket: WebSocket, state: DashboardState) {
    if send_estado(&mut socket, state.vista.estado_actual())
        .await
        .is_err()
    {
        return;
    }

    let mut receiver = state.broadcaster.subscribe();

    while let Ok(estado) = receiver.recv().await {
        if send_estado(&mut socket, estado).await.is_err() {
            break;
        }
    }
}

async fn send_estado(socket: &mut WebSocket, estado: EstadoDashboard) -> Result<()> {
    let payload = serde_json::to_string(&estado)?;
    socket.send(Message::Text(payload.into())).await?;
    Ok(())
}

fn operacion_label(operacion: dominio::Operacion) -> &'static str {
    match operacion {
        dominio::Operacion::Retiro => "retiro",
        dominio::Operacion::Devolucion => "devolucion",
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use dominio::EstadoBicicletas;
    use tokio::sync::broadcast;
    use tower::ServiceExt;

    use super::{
        router, DashboardState, EstadoDashboard, VistaEstadoBicicletas, VistaEstadoMemoria,
    };

    #[test]
    fn serializa_estado_dashboard_con_campos_requeridos() {
        let estado = EstadoDashboard {
            bicicletas_disponibles: 3,
            bicicletas_en_uso: 2,
            actualizado_en: "2026-07-06T12:00:00Z".to_string(),
            estado_conexion: "conectado".to_string(),
            movimiento: None,
        };

        let json = serde_json::to_value(&estado).unwrap();

        assert_eq!(json["bicicletas_disponibles"], 3);
        assert_eq!(json["bicicletas_en_uso"], 2);
        assert_eq!(json["actualizado_en"], "2026-07-06T12:00:00Z");
        assert_eq!(json["estado_conexion"], "conectado");
        assert!(json.get("movimiento").is_some());
    }

    #[tokio::test]
    async fn get_dashboard_retorna_index_con_indicadores_e_historial() {
        let (broadcaster, _) = broadcast::channel(8);
        let app = router(DashboardState {
            vista: Arc::new(VistaEstadoMemoria::new(EstadoBicicletas::new(0, 0))),
            broadcaster,
        });
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/dashboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();

        assert!(html.contains("bicicletas-disponibles"));
        assert!(html.contains("bicicletas-en-uso"));
        assert!(html.contains("historial-eventos"));
    }

    #[test]
    fn vista_estado_memoria_entrega_estado_inicial_para_websocket() {
        let vista = VistaEstadoMemoria::new(EstadoBicicletas::new(2, 5));

        let estado = vista.estado_actual();

        assert_eq!(estado.bicicletas_en_uso, 2);
        assert_eq!(estado.bicicletas_disponibles, 3);
        assert!(estado.movimiento.is_none());
    }
}
