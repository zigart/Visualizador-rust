## ADDED Requirements

### Requirement: Health check
El sistema SHALL exponer `GET /health` retornando `{"version": "<CARGO_PKG_VERSION>"}`.

#### Scenario: Liveness probe
- **WHEN** K8s consulta `/health`
- **THEN** responde 200 con versión

### Requirement: Publicación manual de mensajes
El sistema SHALL exponer `POST /rabbit/mensajes` que publica a RabbitMQ y retorna 202.

#### Scenario: Publicación autenticada
- **GIVEN** headers `X-Usuario` y `X-Contrasena` válidos
- **WHEN** se envía JSON de recorrido
- **THEN** el mensaje se publica al exchange y responde 202

#### Scenario: Publicación sin auth
- **WHEN** faltan credenciales
- **THEN** responde 401

### Requirement: Logging estructurado
El sistema SHALL loguear inicio/fin de procesamiento y errores de dominio con `tracing`.

#### Scenario: Error de dominio logueado
- **WHEN** falla validación de negocio
- **THEN** `tracing::error` incluye `id_recorrido`, `id_usuario`, `operacion` y mensaje de error

### Requirement: Logging JSON en producción
El sistema SHALL soportar `LOG_FORMAT=json` para agregadores.

#### Scenario: Logs JSON
- **WHEN** `LOG_FORMAT=json`
- **THEN** los logs se emiten en formato JSON

### Requirement: Despliegue containerizado
El proyecto SHALL incluir Dockerfile multi-stage y manifests K8s con `livenessProbe` en `/health`.

#### Scenario: Liveness configurado
- **WHEN** se aplica el Deployment de K8s
- **THEN** el contenedor tiene `livenessProbe` HTTP apuntando a `/health`
