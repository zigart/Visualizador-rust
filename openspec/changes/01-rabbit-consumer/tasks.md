## 1. Configuracion

- [x] 1.1 Agregar `QUEUE_NAME` a `AppConfig` con default `bike_trips`.
- [x] 1.2 Agregar `RABBIT_PREFETCH` a `AppConfig` con default `1`.
- [x] 1.3 Documentar variables `QUEUE_NAME` y `RABBIT_PREFETCH` en README y `.env.example`.

## 2. Adaptador RabbitMQ

- [x] 2.1 Crear adaptador `lapin` en `crates/adaptadores/src/rabbitmq.rs`.
- [x] 2.2 Configurar conexion con recuperacion automatica habilitada.
- [x] 2.3 Crear canal, declarar o asegurar la cola configurada y aplicar `basic_qos`.
- [x] 2.4 Implementar loop de consumo con auto-ack deshabilitado.

## 3. ACK/NACK

- [x] 3.1 Definir handler async para procesar payloads recibidos.
- [x] 3.2 Enviar ACK cuando el handler finaliza exitosamente.
- [x] 3.3 Enviar NACK con `requeue=true` cuando el handler retorna error.
- [x] 3.4 Agregar tests unitarios para la decision ACK/NACK sin requerir broker.

## 4. Integracion API

- [x] 4.1 Integrar el consumer en `crates/api/src/main.rs`.
- [x] 4.2 Lanzar el loop de consumo con `tokio::spawn`.
- [x] 4.3 Agregar logging de conexion, suscripcion, ACK, NACK y reconexion.

## 5. Verificacion

- [x] 5.1 Ejecutar `cargo fmt`.
- [x] 5.2 Ejecutar `cargo build`.
- [x] 5.3 Ejecutar `cargo test`.
- [x] 5.4 Validar el change con `openspec validate`.
