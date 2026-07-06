#![forbid(unsafe_code)]

mod settings;

use adaptadores::{
    rabbitmq::{consume_forever, RabbitMqSettings},
    validacion_mensajes::parse_movimiento_payload,
};
use anyhow::Result;
use dominio::Operacion;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = settings::AppConfig::from_env()?;
    init_tracing(&config);

    let rabbit_settings = RabbitMqSettings {
        url: config.rabbitmq_url.clone(),
        queue: config.queue_name.clone(),
        prefetch: config.rabbit_prefetch,
    };

    let consumer = tokio::spawn(async move {
        consume_forever(rabbit_settings, |payload| async move {
            let movimiento = parse_movimiento_payload(&payload)?;
            info!(
                id_recorrido = movimiento.id_recorrido,
                id_usuario = movimiento.id_usuario,
                operacion = %operacion_label(movimiento.operacion),
                fechahora = %movimiento.fechahora,
                "movimiento valido parseado"
            );
            Ok(())
        })
        .await
    });

    info!("visualizador iniciado; presione Ctrl+C para finalizar");
    tokio::signal::ctrl_c().await?;
    consumer.abort();
    info!("visualizador finalizado");
    Ok(())
}

fn init_tracing(config: &settings::AppConfig) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    if config.log_format == "json" {
        fmt().with_env_filter(filter).json().init();
    } else {
        fmt().with_env_filter(filter).pretty().init();
    }
}

fn operacion_label(operacion: Operacion) -> &'static str {
    match operacion {
        Operacion::Retiro => "retiro",
        Operacion::Devolucion => "devolucion",
    }
}
