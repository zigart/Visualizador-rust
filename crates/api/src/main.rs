#![forbid(unsafe_code)]

mod dashboard;
mod operations;
mod settings;

use adaptadores::{
    rabbitmq::{consume_forever, RabbitMqPublisherSettings, RabbitMqSettings},
    validacion_mensajes::parse_movimiento_payload,
};
use anyhow::Result;
use dashboard::{publicar_movimiento_valido, DashboardState, VistaEstadoMemoria};
use dominio::{EstadoBicicletas, Operacion};
use operations::{ManualAuth, OperationsState, RabbitManualPublisher};
use serde_json::Value;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};
use tracing::{error, info};
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
    let operations_state = OperationsState {
        auth: ManualAuth {
            usuario: config.operator_user.clone(),
            contrasena: config.operator_password.clone(),
        },
        publisher: Arc::new(RabbitManualPublisher::new(RabbitMqPublisherSettings {
            url: config.rabbitmq_url.clone(),
            routing_key: config.queue_name.clone(),
        })),
    };
    let app =
        dashboard::router(dashboard_state.clone()).merge(operations::router(operations_state));

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
                let movimiento = match parse_movimiento_payload(&payload) {
                    Ok(movimiento) => movimiento,
                    Err(error) => {
                        log_error_dominio(&payload, &error.to_string());
                        return Err(error.into());
                    }
                };
                let operacion = operacion_label(movimiento.operacion);
                info!(
                    id_recorrido = movimiento.id_recorrido,
                    id_usuario = movimiento.id_usuario,
                    operacion = %operacion,
                    fechahora = %movimiento.fechahora,
                    "inicio procesamiento de movimiento"
                );
                if let Err(error) =
                    publicar_movimiento_valido(&dashboard_state, movimiento.clone()).await
                {
                    error!(
                        id_recorrido = movimiento.id_recorrido,
                        id_usuario = movimiento.id_usuario,
                        operacion = %operacion,
                        error = %error,
                        "error de dominio procesando mensaje"
                    );
                    return Err(error);
                }
                info!(
                    id_recorrido = movimiento.id_recorrido,
                    id_usuario = movimiento.id_usuario,
                    operacion = %operacion,
                    "fin procesamiento de movimiento"
                );
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

fn log_error_dominio(payload: &[u8], mensaje: &str) {
    let value = serde_json::from_slice::<Value>(payload).ok();
    let id_recorrido = value
        .as_ref()
        .and_then(|value| value.get("id_recorrido"))
        .map(Value::to_string);
    let id_usuario = value
        .as_ref()
        .and_then(|value| value.get("id_usuario"))
        .map(Value::to_string);
    let operacion = value
        .as_ref()
        .and_then(|value| value.get("operacion"))
        .map(Value::to_string);

    error!(
        id_recorrido = id_recorrido.as_deref().unwrap_or(""),
        id_usuario = id_usuario.as_deref().unwrap_or(""),
        operacion = operacion.as_deref().unwrap_or(""),
        error = %mensaje,
        "error de dominio procesando mensaje"
    );
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
