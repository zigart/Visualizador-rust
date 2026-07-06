#![forbid(unsafe_code)]

mod dashboard;
mod settings;

use adaptadores::{
    rabbitmq::{consume_forever, RabbitMqSettings},
    validacion_mensajes::parse_movimiento_payload,
};
use anyhow::Result;
use dashboard::{publicar_movimiento_valido, DashboardState, VistaEstadoMemoria};
use dominio::{EstadoBicicletas, Operacion};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};
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

    let (broadcaster, _) = broadcast::channel(128);
    let dashboard_state = DashboardState {
        vista: Arc::new(VistaEstadoMemoria::new(EstadoBicicletas::new(0, 0))),
        broadcaster,
    };
    let app = dashboard::router(dashboard_state.clone());

    let bind: SocketAddr = config.http_bind.parse()?;
    let listener = TcpListener::bind(bind).await?;
    let server = tokio::spawn(async move {
        info!(%bind, "servidor HTTP iniciado");
        if let Err(error) = axum::serve(listener, app).await {
            tracing::error!(error = %error, "servidor HTTP finalizado con error");
        }
    });

    let consumer_dashboard_state = dashboard_state.clone();
    let consumer = tokio::spawn(async move {
        consume_forever(rabbit_settings, move |payload| {
            let dashboard_state = consumer_dashboard_state.clone();
            async move {
                let movimiento = parse_movimiento_payload(&payload)?;
                info!(
                    id_recorrido = movimiento.id_recorrido,
                    id_usuario = movimiento.id_usuario,
                    operacion = %operacion_label(movimiento.operacion),
                    fechahora = %movimiento.fechahora,
                    "movimiento valido parseado"
                );
                publicar_movimiento_valido(&dashboard_state, movimiento).await?;
                Ok(())
            }
        })
        .await
    });

    info!("visualizador iniciado; presione Ctrl+C para finalizar");
    tokio::signal::ctrl_c().await?;
    consumer.abort();
    server.abort();
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
