use std::{future::Future, time::Duration};

use anyhow::{Context, Result};
use futures_lite::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Connection, ConnectionProperties,
};
use tokio::time::sleep;
use tracing::{error, info, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RabbitMqSettings {
    pub url: String,
    pub queue: String,
    pub prefetch: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrokerConfirmation {
    Ack,
    NackRequeue,
}

pub fn confirmation_for_result<T, E>(result: &Result<T, E>) -> BrokerConfirmation {
    if result.is_ok() {
        BrokerConfirmation::Ack
    } else {
        BrokerConfirmation::NackRequeue
    }
}

pub async fn consume_forever<H, Fut>(settings: RabbitMqSettings, handler: H) -> !
where
    H: Fn(Vec<u8>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    loop {
        if let Err(error) = consume_once(settings.clone(), handler.clone()).await {
            warn!(
                error = %error,
                queue = %settings.queue,
                "consumer RabbitMQ detenido; reintentando conexion"
            );
            sleep(Duration::from_secs(2)).await;
        }
    }
}

async fn consume_once<H, Fut>(settings: RabbitMqSettings, handler: H) -> Result<()>
where
    H: Fn(Vec<u8>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let connection = Connection::connect(
        &settings.url,
        ConnectionProperties::default().with_connection_name("visualizador-api".into()),
    )
    .await
    .with_context(|| format!("no se pudo conectar a RabbitMQ en {}", settings.url))?;

    info!(queue = %settings.queue, "conexion RabbitMQ establecida");

    let channel = connection
        .create_channel()
        .await
        .context("no se pudo crear canal RabbitMQ")?;

    channel
        .queue_declare(
            &settings.queue,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await
        .with_context(|| format!("no se pudo declarar cola {}", settings.queue))?;

    channel
        .basic_qos(settings.prefetch, BasicQosOptions::default())
        .await
        .with_context(|| format!("no se pudo configurar prefetch {}", settings.prefetch))?;

    let mut consumer = channel
        .basic_consume(
            &settings.queue,
            "visualizador-api",
            BasicConsumeOptions {
                no_ack: false,
                ..BasicConsumeOptions::default()
            },
            FieldTable::default(),
        )
        .await
        .with_context(|| format!("no se pudo consumir cola {}", settings.queue))?;

    info!(
        queue = %settings.queue,
        prefetch = settings.prefetch,
        "consumer RabbitMQ suscripto"
    );

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.context("error recibiendo mensaje RabbitMQ")?;
        let payload = delivery.data.clone();
        let processed = handler(payload).await;

        match confirmation_for_result(&processed) {
            BrokerConfirmation::Ack => {
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .context("fallo enviando ACK a RabbitMQ")?;
                info!("mensaje RabbitMQ confirmado con ACK");
            }
            BrokerConfirmation::NackRequeue => {
                if let Err(error) = &processed {
                    error!(error = %error, "fallo el procesamiento del mensaje RabbitMQ");
                }

                delivery
                    .nack(BasicNackOptions {
                        multiple: false,
                        requeue: true,
                    })
                    .await
                    .context("fallo enviando NACK a RabbitMQ")?;
                warn!("mensaje RabbitMQ rechazado con NACK requeue=true");
            }
        }
    }

    Err(anyhow::anyhow!("stream de consumer RabbitMQ finalizado"))
}

#[cfg(test)]
mod tests {
    use super::{confirmation_for_result, BrokerConfirmation};

    #[test]
    fn confirma_ack_cuando_el_resultado_es_exitoso() {
        let result: anyhow::Result<()> = Ok(());

        assert_eq!(confirmation_for_result(&result), BrokerConfirmation::Ack);
    }

    #[test]
    fn confirma_nack_requeue_cuando_el_resultado_es_error() {
        let result: anyhow::Result<()> = Err(anyhow::anyhow!("fallo"));

        assert_eq!(
            confirmation_for_result(&result),
            BrokerConfirmation::NackRequeue
        );
    }
}
