## Why

Visualizador necesita iniciar como un servicio Rust mantenible para observar en tiempo real los retiros y devoluciones de bicicletas publicas. El sistema debe consumir eventos ya publicados por RabbitMQ, aplicar reglas de negocio, persistir estado en PostgreSQL y exponer un dashboard vivo por WebSocket.

## What Changes

- Crear un workspace Cargo funcional con crates `dominio`, `adaptadores` y `api`.
- Definir una arquitectura hexagonal donde `dominio` contiene tipos, reglas y puertos, `adaptadores` implementa integraciones externas, y `api` compone el servicio HTTP/WebSocket.
- Agregar consumo RabbitMQ desde la cola `bike_trips` para eventos de retiro y devolucion.
- Agregar persistencia PostgreSQL mediante `sqlx` para recorridos y estado actual de bicicletas.
- Agregar dashboard en tiempo real usando WebSocket nativo con `axum`.
- Agregar `docker-compose.yml` con PostgreSQL y RabbitMQ para desarrollo local.
- Agregar migraciones SQL iniciales y README con instrucciones para build, test, migraciones y ejecucion local.

## Capabilities

### New Capabilities

- `event-ingestion`: Consumo de eventos de retiro y devolucion desde RabbitMQ.
- `trip-domain`: Reglas de negocio y modelo de dominio para recorridos y estado de bicicletas.
- `postgres-persistence`: Persistencia de recorridos y estado actual de bicicletas en PostgreSQL.
- `realtime-dashboard`: Exposicion de dashboard en tiempo real por WebSocket nativo con `axum`.
- `developer-workspace`: Workspace Cargo, entorno Docker y documentacion de desarrollo.

### Modified Capabilities

- None.

## Impact

- Nuevo workspace Rust con crates `dominio`, `adaptadores` y `api`.
- Nuevas dependencias principales: `tokio`, `axum`, `lapin`, `sqlx`, `serde`, `thiserror` y `tracing`.
- Nuevos servicios locales PostgreSQL y RabbitMQ via Docker Compose.
- Nuevas tablas `recorridos` y `estado_bicicletas`.
- Nuevo contrato WebSocket para publicar actualizaciones de estado al dashboard.
