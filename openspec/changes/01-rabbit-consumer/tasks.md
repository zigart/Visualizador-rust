## 1. Configuracion

- [ ] 1.1 Agregar `QUEUE_NAME` a `AppConfig` con default `bike_trips`.
- [ ] 1.2 Agregar `RABBIT_PREFETCH` a `AppConfig` con default `1`.
- [ ] 1.3 Documentar variables `QUEUE_NAME` y `RABBIT_PREFETCH` en README y `.env.example`.

## 2. Adaptador RabbitMQ

- [ ] 2.1 Crear adaptador `lapin` en `crates/adaptadores/src/rabbitmq.rs`.
- [ ] 2.2 Configurar conexion con recuperacion automatica habilitada.
- [ ] 2.3 Crear canal, declarar o asegurar la cola configurada y aplicar `basic_qos`.
- [ ] 2.4 Implementar loop de consumo con auto-ack deshabilitado.

## 3. ACK/NACK

- [ ] 3.1 Definir handler async para procesar payloads recibidos.
- [ ] 3.2 Enviar ACK cuando el handler finaliza exitosamente.
- [ ] 3.3 Enviar NACK con `requeue=true` cuando el handler retorna error.
- [ ] 3.4 Agregar tests unitarios para la decision ACK/NACK sin requerir broker.

## 4. Integracion API

- [ ] 4.1 Integrar el consumer en `crates/api/src/main.rs`.
- [ ] 4.2 Lanzar el loop de consumo con `tokio::spawn`.
- [ ] 4.3 Agregar logging de conexion, suscripcion, ACK, NACK y reconexion.

## 5. Verificacion

- [ ] 5.1 Ejecutar `cargo fmt`.
- [ ] 5.2 Ejecutar `cargo build`.
- [ ] 5.3 Ejecutar `cargo test`.
- [ ] 5.4 Validar el change con `openspec validate`.
