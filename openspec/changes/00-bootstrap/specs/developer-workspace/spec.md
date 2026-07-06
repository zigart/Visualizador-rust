## ADDED Requirements

### Requirement: Proveer workspace Cargo funcional
El proyecto SHALL incluir un workspace Cargo con crates `dominio`, `adaptadores` y `api` que compile y ejecute tests.

#### Scenario: Build del workspace
- **WHEN** se ejecuta `cargo build` desde la raiz del repositorio
- **THEN** todos los crates del workspace compilan correctamente

#### Scenario: Tests del workspace
- **WHEN** se ejecuta `cargo test` desde la raiz del repositorio
- **THEN** los tests del workspace finalizan correctamente

### Requirement: Proveer servicios locales con Docker Compose
El proyecto SHALL incluir `docker-compose.yml` con PostgreSQL y RabbitMQ configurados para desarrollo local.

#### Scenario: Servicios levantados
- **WHEN** se ejecuta Docker Compose para el entorno local
- **THEN** PostgreSQL y RabbitMQ quedan disponibles para la API

### Requirement: Documentar desarrollo local
El proyecto SHALL incluir README con instrucciones para instalar dependencias, levantar servicios, aplicar migraciones, ejecutar tests y correr la API.

#### Scenario: Desarrollador nuevo
- **WHEN** un desarrollador sigue el README desde un entorno limpio
- **THEN** puede construir, probar y ejecutar Visualizador localmente
