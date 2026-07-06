use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    pub database_url: String,
    pub rabbitmq_url: String,
    pub queue_name: String,
    pub rabbit_prefetch: u16,
    pub http_bind: String,
    pub app_env: String,
    pub log_format: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        config::Config::builder()
            .set_default(
                "database_url",
                "postgres://visualizador:visualizador@localhost:5432/visualizador",
            )?
            .set_default(
                "rabbitmq_url",
                "amqp://visualizador:visualizador@localhost:5672/%2f",
            )?
            .set_default("queue_name", "bike_trips")?
            .set_default("rabbit_prefetch", 1)?
            .set_default("http_bind", "127.0.0.1:3000")?
            .set_default("app_env", "dev")?
            .set_default("log_format", "pretty")?
            .add_source(config::Environment::default())
            .build()
            .context("no se pudo cargar la configuracion")?
            .try_deserialize()
            .context("no se pudo deserializar AppConfig")
    }
}
