## Why

Hoy los mensajes con error de validacion (parseo JSON, campos invalidos o reglas de negocio) reciben NACK con requeue, lo que puede generar reintentos infinitos sobre mensajes irrecuperables y presionar la cola. Ademas, esos errores deben quedar registrados en Sumo Logic para observabilidad operativa, con fallback a consola cuando el destino remoto no este disponible.

## What Changes

- Confirmar (ACK) mensajes RabbitMQ cuando falle la **validacion** del payload o las reglas de negocio de dominio, en lugar de NACK con requeue.
- Mantener NACK solo para fallos transitorios de infraestructura (por ejemplo, error de base de datos o broker).
- Loguear cada error de validacion con campos estructurados (`id_recorrido`, `id_usuario`, `operacion`, tipo de error, payload resumido).
- Enviar logs de error a Sumo Logic cuando este configurado; si el envio falla o no hay configuracion, emitir el mismo evento por consola (stdout/stderr).
- **BREAKING**: cambia el contrato de confirmacion ante errores de validacion respecto de `rabbit-consumer` y `message-validation`.

## Capabilities

### New Capabilities

- `sumologic-logging`: Envio de logs estructurados a Sumo Logic con fallback a consola.

### Modified Capabilities

- `rabbit-consumer`: ACK en errores de validacion; NACK reservado a fallos transitorios.
- `message-validation`: escenarios de rechazo actualizados a ACK + log en lugar de NACK.
- `operations`: requisitos de logging extendidos para incluir destino Sumo Logic y fallback.

## Impact

- Actualiza `crates/adaptadores/src/rabbitmq.rs` (politica ACK/NACK y clasificacion de errores).
- Actualiza handler del consumer en `crates/api/src/main.rs`.
- Agrega configuracion de entorno para Sumo Logic (endpoint, credenciales o collector URL).
- Extiende inicializacion de `tracing-subscriber` en `crates/api/src/settings.rs` y `main.rs`.
- Actualiza README y `.env.example` con variables de observabilidad.
- Tests unitarios de `confirmation_for_result` y tests de integracion del consumer.
