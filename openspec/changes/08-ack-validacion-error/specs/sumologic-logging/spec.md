## ADDED Requirements

### Requirement: Envio de logs a Sumo Logic
El sistema SHALL enviar logs de error de validacion a Sumo Logic cuando `SUMOLOGIC_ENDPOINT` este configurado.

#### Scenario: Error de validacion enviado a Sumo Logic
- **GIVEN** `SUMOLOGIC_ENDPOINT` configurado y disponible
- **WHEN** falla la validacion de un mensaje RabbitMQ
- **THEN** se emite un evento estructurado JSON hacia Sumo Logic con tipo de error y campos de contexto

### Requirement: Fallback a consola
El sistema SHALL emitir por consola (stdout/stderr) los mismos eventos de error cuando Sumo Logic no este configurado o falle el envio.

#### Scenario: Sumo Logic no configurado
- **GIVEN** `SUMOLOGIC_ENDPOINT` vacio o ausente
- **WHEN** falla la validacion de un mensaje
- **THEN** el error se loguea en consola con `tracing::error` en formato JSON o pretty segun `LOG_FORMAT`

#### Scenario: Sumo Logic no disponible
- **GIVEN** `SUMOLOGIC_ENDPOINT` configurado pero el endpoint no responde
- **WHEN** falla el envio remoto
- **THEN** el mismo evento se escribe en consola sin interrumpir el procesamiento del mensaje

### Requirement: Campos estructurados en logs de validacion
Los logs de error de validacion SHALL incluir `id_recorrido`, `id_usuario`, `operacion`, `error` y `origen` (`parseo` o `dominio`) cuando esten disponibles.

#### Scenario: Log de parseo invalido
- **WHEN** el payload no es JSON valido
- **THEN** el log incluye `origen=parseo` y el detalle del error

#### Scenario: Log de regla de negocio
- **WHEN** `SistemaBicicletas` rechaza un movimiento
- **THEN** el log incluye `origen=dominio`, `id_recorrido`, `id_usuario` y `operacion`
