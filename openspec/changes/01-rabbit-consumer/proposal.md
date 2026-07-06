## Why

Visualizador necesita empezar a consumir eventos reales desde RabbitMQ para conectar el bootstrap con el flujo de datos del sistema. Este change agrega el consumer con `lapin`, manteniendo confirmacion manual, reencolado ante errores y reconexion automatica.

## What Changes

- Agregar adaptador RabbitMQ basado en `lapin`.
- Consumir desde la cola configurada por `QUEUE_NAME`, con default `bike_trips`.
- Procesar mensajes en un loop async iniciado desde `api` con `tokio::spawn`.
- Confirmar mensajes con ACK solo tras procesamiento exitoso.
- Enviar NACK con `requeue=true` cuando falle el parseo o procesamiento.
- Configurar prefetch mediante `RABBIT_PREFETCH`, con default `1`.

## Capabilities

### New Capabilities

- `rabbit-consumer`: Consumer RabbitMQ con suscripcion, ACK/NACK manual, reconexion automatica y prefetch configurable.

### Modified Capabilities

- None.

## Impact

- Actualiza `crates/adaptadores` con el consumer `lapin`.
- Actualiza `crates/api` para configurar y lanzar el consumer en una task Tokio.
- Agrega pruebas de comportamiento para ACK/NACK, configuracion y parseo donde sea viable sin broker real.
