## 1. Workspace y dependencias

- [ ] 1.1 Crear `Cargo.toml` raiz con workspace para `crates/dominio`, `crates/adaptadores` y `crates/api`.
- [ ] 1.2 Crear los tres crates con estructura inicial de `src/lib.rs` o `src/main.rs` segun corresponda.
- [ ] 1.3 Configurar dependencias compartidas: `tokio`, `axum`, `lapin`, `sqlx`, `serde`, `thiserror`, `tracing` y utilidades necesarias.
- [ ] 1.4 Verificar que `cargo build` compila el workspace vacio.

## 2. Dominio

- [ ] 2.1 Definir tipos de dominio para bicicletas, estaciones, recorridos, eventos de retiro/devolucion y timestamps.
- [ ] 2.2 Definir errores de negocio en espanol con `thiserror`.
- [ ] 2.3 Definir traits de puertos para repositorio de recorridos, estado de bicicletas y publicacion de actualizaciones.
- [ ] 2.4 Implementar servicios de dominio para registrar retiro y devolucion usando los puertos.
- [ ] 2.5 Agregar tests unitarios para retiro valido, retiro duplicado, devolucion valida y devolucion sin recorrido activo.

## 3. PostgreSQL y migraciones

- [ ] 3.1 Agregar directorio de migraciones `migrations/` compatible con `sqlx`.
- [ ] 3.2 Crear migracion inicial para tablas `recorridos` y `estado_bicicletas`.
- [ ] 3.3 Implementar adaptador PostgreSQL para los puertos de dominio.
- [ ] 3.4 Asegurar operaciones transaccionales para abrir/cerrar recorridos y actualizar estado actual.
- [ ] 3.5 Agregar tests de persistencia para migraciones y operaciones principales.

## 4. RabbitMQ

- [ ] 4.1 Definir DTOs JSON de eventos externos de retiro y devolucion en `adaptadores`.
- [ ] 4.2 Implementar consumidor `lapin` para la cola `bike_trips`.
- [ ] 4.3 Mapear mensajes RabbitMQ a comandos o eventos de dominio.
- [ ] 4.4 Confirmar mensajes solo despues de procesamiento y persistencia exitosos.
- [ ] 4.5 Agregar tests para deserializacion y manejo de mensajes validos e invalidos.

## 5. API y WebSocket

- [ ] 5.1 Implementar configuracion de la API por variables de entorno.
- [ ] 5.2 Crear servidor `axum` con endpoint de salud y endpoint WebSocket nativo.
- [ ] 5.3 Implementar canal broadcast en memoria para actualizaciones de dashboard.
- [ ] 5.4 Integrar dominio, PostgreSQL, RabbitMQ y broadcast en el binario `api`.
- [ ] 5.5 Agregar tests para conexion WebSocket y emision de actualizaciones.

## 6. Entorno local y documentacion

- [ ] 6.1 Crear `docker-compose.yml` con PostgreSQL y RabbitMQ para desarrollo local.
- [ ] 6.2 Documentar variables de entorno requeridas y valores default.
- [ ] 6.3 Actualizar README con instrucciones para levantar servicios, aplicar migraciones, ejecutar tests y correr la API.
- [ ] 6.4 Documentar el formato esperado de mensajes RabbitMQ.

## 7. Verificacion final

- [ ] 7.1 Ejecutar `cargo fmt` sobre el workspace.
- [ ] 7.2 Ejecutar `cargo build` desde la raiz.
- [ ] 7.3 Ejecutar `cargo test` desde la raiz.
- [ ] 7.4 Validar que las specs OpenSpec del change pasan `openspec validate`.
