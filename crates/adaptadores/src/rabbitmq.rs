use std::{future::Future, time::Duration};

use anyhow::{Context, Result};
use futures_lite::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions,
        BasicQosOptions, QueueDeclareOptions,
    },
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use tokio::time::sleep;
use tracing::{error, info, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RabbitMqSettings {
    pub url: String,
    pub queue: String,
    pub prefetch: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RabbitMqPublisherSettings {
    pub url: String,
    pub routing_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrokerConfirmation {
    Ack,
    NackRequeue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingOutcome {
    Success,
    ValidationError,
    TransientError,
}

pub fn confirmation_for_outcome(outcome: ProcessingOutcome) -> BrokerConfirmation {
    match outcome {
        ProcessingOutcome::Success | ProcessingOutcome::ValidationError => BrokerConfirmation::Ack,
        ProcessingOutcome::TransientError => BrokerConfirmation::NackRequeue,
    }
}

pub async fn consume_forever<H, Fut>(settings: RabbitMqSettings, handler: H) -> !
where
    H: Fn(Vec<u8>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ProcessingOutcome> + Send + 'static,
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

pub async fn publish_message(settings: &RabbitMqPublisherSettings, payload: Vec<u8>) -> Result<()> {
    let connection = Connection::connect(
        &settings.url,
        ConnectionProperties::default()
            .with_connection_name("visualizador-manual-publisher".into()),
    )
    .await
    .with_context(|| format!("no se pudo conectar a RabbitMQ en {}", settings.url))?;

    let channel = connection
        .create_channel()
        .await
        .context("no se pudo crear canal RabbitMQ para publicacion")?;

    channel
        .queue_declare(
            &settings.routing_key,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await
        .with_context(|| format!("no se pudo declarar cola {}", settings.routing_key))?;

    channel
        .basic_publish(
            "",
            &settings.routing_key,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default().with_content_type("application/json".into()),
        )
        .await
        .context("fallo publicando mensaje RabbitMQ")?
        .await
        .context("RabbitMQ no confirmo la publicacion")?;

    info!(routing_key = %settings.routing_key, "mensaje publicado manualmente en RabbitMQ");
    Ok(())
}

async fn consume_once<H, Fut>(settings: RabbitMqSettings, handler: H) -> Result<()>
where
    H: Fn(Vec<u8>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ProcessingOutcome> + Send + 'static,
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
        let outcome = handler(payload).await;

        match confirmation_for_outcome(outcome) {
            BrokerConfirmation::Ack => {
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .context("fallo enviando ACK a RabbitMQ")?;
                if outcome == ProcessingOutcome::ValidationError {
                    warn!("mensaje RabbitMQ confirmado con ACK tras error de validacion");
                } else {
                    info!("mensaje RabbitMQ confirmado con ACK");
                }
            }
            BrokerConfirmation::NackRequeue => {
                error!("fallo transitorio procesando mensaje RabbitMQ; NACK requeue");
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
    use super::{confirmation_for_outcome, BrokerConfirmation, ProcessingOutcome};

    #[test]
    fn confirma_ack_cuando_el_procesamiento_es_exitoso() {
        assert_eq!(
            confirmation_for_outcome(ProcessingOutcome::Success),
            BrokerConfirmation::Ack
        );
    }

    #[test]
    fn confirma_ack_cuando_hay_error_de_validacion() {
        assert_eq!(
            confirmation_for_outcome(ProcessingOutcome::ValidationError),
            BrokerConfirmation::Ack
        );
    }

    #[test]
    fn confirma_nack_requeue_cuando_hay_error_transitorio() {
        assert_eq!(
            confirmation_for_outcome(ProcessingOutcome::TransientError),
            BrokerConfirmation::NackRequeue
        );
    }
}
