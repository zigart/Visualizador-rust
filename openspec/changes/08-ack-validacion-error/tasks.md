## 1. Clasificacion de errores

- [x] 1.1 Definir `ProcessingOutcome` o tipo equivalente (`Success`, `ValidationError`, `TransientError`) en adaptadores o api.
- [x] 1.2 Mapear errores de parseo y `ErrorPersistencia::Dominio` a `ValidationError`.
- [x] 1.3 Mapear errores de base de datos y broker a `TransientError`.

## 2. Politica ACK/NACK

- [x] 2.1 Reemplazar `confirmation_for_result` por confirmacion basada en `ProcessingOutcome`.
- [x] 2.2 ACK en `Success` y `ValidationError`; NACK requeue solo en `TransientError`.
- [x] 2.3 Actualizar tests unitarios de `rabbitmq.rs` para los tres escenarios.

## 3. Consumer y logging de validacion

- [x] 3.1 Ajustar handler en `main.rs` para retornar outcome de validacion sin NACK.
- [x] 3.2 Unificar `log_error_dominio` y logs de parseo con campos `origen`, `id_recorrido`, `id_usuario`, `operacion`.
- [x] 3.3 Asegurar que errores de validacion no interrumpen el loop del consumer.

## 4. Sumo Logic

- [x] 4.1 Agregar `SUMOLOGIC_ENDPOINT` y opcionales a `AppConfig` y `.env.example`.
- [x] 4.2 Implementar layer HTTP de `tracing-subscriber` para enviar eventos ERROR a Sumo Logic.
- [x] 4.3 Implementar fallback a consola cuando el endpoint falte o el POST falle.
- [x] 4.4 Documentar configuracion en README.

## 5. Verificacion

- [x] 5.1 Ejecutar `cargo fmt`.
- [x] 5.2 Ejecutar `cargo build`.
- [x] 5.3 Ejecutar `cargo test`.
- [x] 5.4 Validar el change con `openspec validate`.
