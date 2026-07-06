## Context

El bootstrap ya define workspace, configuracion base y dependencia `lapin`. Falta implementar el consumer RabbitMQ que lea eventos desde la cola del productor externo y ejecute un handler de procesamiento sin acoplar RabbitMQ al dominio.

## Goals / Non-Goals

**Goals:**

- Conectar a RabbitMQ al iniciar el servicio.
- Consumir mensajes desde `QUEUE_NAME`, default `bike_trips`.
- Usar ACK manual solo despues de procesamiento exitoso.
- Usar NACK con `requeue=true` ante errores de parseo o dominio.
- Habilitar reconexion automatica de `lapin`.
- Aplicar `basic_qos` con `RABBIT_PREFETCH`, default `1`.
- Integrar el loop de consumo en `api` con `tokio::spawn`.

**Non-Goals:**

- No implementar persistencia final de recorridos en este change.
- No implementar reglas completas de retiro/devolucion si no existen todavia.
- No exponer dashboard WebSocket en este change.

## Decisions

### Adaptador RabbitMQ en `crates/adaptadores`

El consumer vivira en `adaptadores::rabbitmq` y expondra una funcion async que recibe configuracion y un handler de mensajes. El handler sera una abstraccion para que `api` pueda componer procesamiento sin que `dominio` dependa de `lapin`.

### ACK/NACK manual

El consumer deshabilitara auto-ack. Si el handler retorna `Ok(())`, se enviara ACK. Si retorna error, se enviara NACK con `requeue=true`.

### Reconexión

La conexion `lapin` se configurara con auto recovery habilitado. Si el broker cae y vuelve, el consumer debe recuperar la conexion y reanudar la suscripcion.

### Prefetch

El canal aplicara `basic_qos` con `RABBIT_PREFETCH`, default `1`, para limitar mensajes no confirmados y mantener backpressure.

## Risks / Trade-offs

- Reencolar errores de parseo puede generar retry infinito -> se acepta por ahora porque la spec pide `requeue=true`; una evolucion futura puede agregar DLQ.
- Auto recovery depende del comportamiento de `lapin` y del estado del canal -> agregar logs claros y tests de integracion cuando el broker este disponible.
- Sin persistencia final, el handler inicial puede ser un stub verificable -> mantener el contrato para conectar dominio/persistencia en cambios siguientes.
