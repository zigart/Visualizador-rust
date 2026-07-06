## ADDED Requirements

### Requirement: Cargo Workspace
El proyecto SHALL organizarse como workspace Cargo con crates separados `dominio`, `adaptadores` y `api`.

#### Scenario: Build exitoso
- **GIVEN** el workspace Cargo esta definido en la raiz del proyecto
- **WHEN** se ejecuta `cargo build` en la raiz
- **THEN** compilan los tres crates sin errores

#### Scenario: Dominio aislado
- **GIVEN** el crate `dominio` contiene las reglas y tipos centrales del sistema
- **WHEN** se inspecciona el `Cargo.toml` de `dominio`
- **THEN** no depende de `axum`, `lapin` ni `sqlx`

### Requirement: Entorno local reproducible
El proyecto SHALL incluir `docker-compose.yml` con PostgreSQL y RabbitMQ.

#### Scenario: Servicios disponibles
- **GIVEN** Docker esta disponible en el entorno de desarrollo
- **WHEN** se ejecuta `docker compose up -d`
- **THEN** PostgreSQL escucha en `5432` y RabbitMQ en `5672`

### Requirement: Esquema inicial de base de datos
El proyecto SHALL incluir migraciones para tablas `recorridos` y `estado_bicicletas` con seed de estado en cero.

#### Scenario: Migracion aplicada
- **GIVEN** PostgreSQL esta disponible y la base de datos no tiene el esquema de Visualizador
- **WHEN** se ejecuta `sqlx migrate run`
- **THEN** existen las tablas `recorridos` y `estado_bicicletas` con fila singleton `id=1`
