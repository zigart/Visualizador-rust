# Visualizador Rust

## Producto

Visualizador es un microservicio de visualizacion en tiempo real del sistema de bicicletas publicas.

## Stack

- Rust async con `tokio`.
- API HTTP y WebSocket con `axum`.
- Consumo de RabbitMQ con `lapin`.
- Persistencia PostgreSQL con `sqlx`.
- Observabilidad con `tracing`.
- Serializacion con `serde`.
- Errores con `thiserror`.

## Arquitectura

- Workspace Cargo con crates separados.
- `dominio`: reglas y tipos de dominio, sin dependencias de infraestructura.
- `adaptadores`: integraciones externas e infraestructura.
- `api`: binario HTTP/WebSocket y composicion de la aplicacion.

## Integracion externa

Consume la cola RabbitMQ `bike_trips`, publicada por un servicio Procesador externo.

## Fuera de alcance

- Ingesta CSV.
- Validacion de estaciones.
- Publicacion original a RabbitMQ.

## Convenciones

- Nombres y errores de dominio en espanol.
- Tests con `cargo test` y `testcontainers`.
- Puerto default: `3000`.
