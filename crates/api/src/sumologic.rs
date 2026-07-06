use std::sync::Arc;

use chrono::Utc;
use reqwest::Client;
use serde_json::{json, Value};
use tracing::field::{Field, Visit};
use tracing::Level;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

#[derive(Clone)]
pub struct SumoLogicLayer {
    client: Client,
    endpoint: Arc<str>,
    source_name: Arc<str>,
}

impl SumoLogicLayer {
    pub fn new(endpoint: String, source_name: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(2))
                .build()
                .unwrap_or_else(|_| Client::new()),
            endpoint: Arc::from(endpoint),
            source_name: Arc::from(source_name),
        }
    }

    fn should_forward(event: &tracing::Event<'_>) -> bool {
        event.metadata().level() == &Level::ERROR
    }

    fn build_payload(&self, event: &tracing::Event<'_>) -> Value {
        let mut visitor = EventVisitor::default();
        event.record(&mut visitor);

        let message = visitor.message.clone();
        let error = visitor.error.or(message.clone());

        json!({
            "service": "visualizador-rust",
            "level": event.metadata().level().to_string().to_lowercase(),
            "event": visitor.event.unwrap_or_else(|| "log".to_string()),
            "origen": visitor.origen,
            "id_recorrido": visitor.id_recorrido,
            "id_usuario": visitor.id_usuario,
            "operacion": visitor.operacion,
            "error": error,
            "message": message,
            "timestamp": Utc::now().to_rfc3339(),
            "source": self.source_name.as_ref(),
        })
    }

    fn send_async(&self, payload: Value) {
        let client = self.client.clone();
        let endpoint = self.endpoint.clone();
        let source_name = self.source_name.clone();

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                if let Err(error) = client.post(endpoint.as_ref()).json(&payload).send().await {
                    tracing::warn!(
                        error = %error,
                        source = %source_name,
                        "fallo envio de log a Sumo Logic; evento disponible en consola"
                    );
                }
            });
        }
    }
}

impl<S> Layer<S> for SumoLogicLayer
where
    S: tracing::Subscriber,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        if !Self::should_forward(event) {
            return;
        }

        let payload = self.build_payload(event);
        self.send_async(payload);
    }
}

#[derive(Default)]
struct EventVisitor {
    event: Option<String>,
    origen: Option<String>,
    id_recorrido: Option<String>,
    id_usuario: Option<String>,
    operacion: Option<String>,
    error: Option<String>,
    message: Option<String>,
}

impl Visit for EventVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.record_str(field, &format!("{value:?}"));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "event" => self.event = Some(value.to_string()),
            "origen" => self.origen = Some(value.to_string()),
            "id_recorrido" => self.id_recorrido = Some(value.to_string()),
            "id_usuario" => self.id_usuario = Some(value.to_string()),
            "operacion" => self.operacion = Some(value.to_string()),
            "error" => self.error = Some(value.to_string()),
            "message" => self.message = Some(value.to_string()),
            _ => {}
        }
    }
}
