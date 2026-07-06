use std::{future::Future, pin::Pin, sync::Arc};

use adaptadores::rabbitmq::{publish_message, RabbitMqPublisherSettings};
use anyhow::Result;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct OperationsState {
    pub auth: ManualAuth,
    pub publisher: Arc<dyn ManualPublisher>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualAuth {
    pub usuario: String,
    pub contrasena: String,
}

pub trait ManualPublisher: Send + Sync {
    fn publish<'a>(
        &'a self,
        payload: Vec<u8>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;
}

#[derive(Debug, Clone)]
pub struct RabbitManualPublisher {
    settings: RabbitMqPublisherSettings,
}

impl RabbitManualPublisher {
    pub fn new(settings: RabbitMqPublisherSettings) -> Self {
        Self { settings }
    }
}

impl ManualPublisher for RabbitManualPublisher {
    fn publish<'a>(
        &'a self,
        payload: Vec<u8>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { publish_message(&self.settings, payload).await })
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    version: &'static str,
}

pub fn router(state: OperationsState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/rabbit/mensajes", post(publicar_mensaje))
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn publicar_mensaje(
    State(state): State<OperationsState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !credenciales_validas(&headers, &state.auth) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "unauthorized" })),
        )
            .into_response();
    }

    let value: Value = match serde_json::from_slice(&body) {
        Ok(value) => value,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "json_invalido", "detalle": error.to_string() })),
            )
                .into_response();
        }
    };

    let payload = match serde_json::to_vec(&value) {
        Ok(payload) => payload,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "json_invalido", "detalle": error.to_string() })),
            )
                .into_response();
        }
    };

    match state.publisher.publish(payload).await {
        Ok(()) => (StatusCode::ACCEPTED, Json(json!({ "estado": "publicado" }))).into_response(),
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({ "error": "rabbitmq_publish_failed", "detalle": error.to_string() })),
        )
            .into_response(),
    }
}

fn credenciales_validas(headers: &HeaderMap, auth: &ManualAuth) -> bool {
    let usuario = headers
        .get("X-Usuario")
        .and_then(|value| value.to_str().ok());
    let contrasena = headers
        .get("X-Contrasena")
        .and_then(|value| value.to_str().ok());

    usuario == Some(auth.usuario.as_str()) && contrasena == Some(auth.contrasena.as_str())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use anyhow::Result;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use tower::{Service, ServiceExt};

    use super::{router, ManualAuth, ManualPublisher, OperationsState};

    #[derive(Default)]
    struct RecordingPublisher {
        payloads: Mutex<Vec<Vec<u8>>>,
    }

    impl ManualPublisher for RecordingPublisher {
        fn publish<'a>(
            &'a self,
            payload: Vec<u8>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
            Box::pin(async move {
                self.payloads.lock().unwrap().push(payload);
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn health_responde_version() {
        let app = router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["version"], env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn publicar_mensaje_sin_auth_responde_401() {
        let app = router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/rabbit/mensajes")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"id_recorrido":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn publicar_mensaje_con_auth_responde_202() {
        let mut app = router(test_state());
        let request = Request::builder()
            .method("POST")
            .uri("/rabbit/mensajes")
            .header("content-type", "application/json")
            .header("X-Usuario", "operador")
            .header("X-Contrasena", "secreto")
            .body(Body::from(r#"{"id_recorrido":1}"#))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }

    fn test_state() -> OperationsState {
        OperationsState {
            auth: ManualAuth {
                usuario: "operador".to_string(),
                contrasena: "secreto".to_string(),
            },
            publisher: Arc::new(RecordingPublisher::default()),
        }
    }
}
