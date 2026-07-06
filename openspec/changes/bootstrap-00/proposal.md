## Why

Visualizador necesita iniciar como un servicio Rust mantenible para observar en tiempo real los retiros y devoluciones de bicicletas. El sistema debe consumir eventos desde RabbitMQ, aplicar reglas de negocio, persistir estado en PostgreSQL y publicar actualizaciones vivas por WebSocket.

## What Changes

- Crear un workspace Cargo funcional con crates `dominio`, `adaptadores` y `api`.
- Definir una arquitectura hexagonal donde `dominio` contiene tipos, reglas y puertos, `adaptadores` implementa integraciones externas, y `api` compone el servicio ejecutable.
- Consumir eventos JSON de retiro y devolucion desde RabbitMQ.
- Persistir recorridos y estado actual de bicicletas en PostgreSQL mediante `sqlx`.
- Exponer un dashboard en tiempo real usando WebSocket nativo con `axum`.
- Agregar `docker-compose.yml` con PostgreSQL y RabbitMQ para desarrollo local.
- Agregar migraciones SQL iniciales y README con instrucciones de desarrollo.

## Capabilities

### New Capabilities

- `developer-workspace`: Workspace Cargo, entorno Docker, migraciones y documentacion para desarrollar Visualizador.
- `trip-domain`: Modelo de dominio, reglas de negocio y puertos para recorridos y estado de bicicletas.
- `event-ingestion`: Consumo de eventos de retiro y devolucion desde RabbitMQ.
- `postgres-persistence`: Persistencia PostgreSQL para recorridos y estado actual de bicicletas.
- `realtime-dashboard`: API HTTP y dashboard en tiempo real por WebSocket nativo con `axum`.

### Modified Capabilities

- None.

## Impact

- Nuevo workspace Rust con crates `dominio`, `adaptadores` y `api`.
- Nuevas dependencias principales: `tokio`, `axum`, `lapin`, `sqlx`, `serde`, `thiserror` y `tracing`.
- Nuevos servicios locales PostgreSQL y RabbitMQ via Docker Compose.
- Nuevas tablas `recorridos` y `estado_bicicletas`.
- Nuevo contrato WebSocket para emitir actualizaciones de dashboard.
