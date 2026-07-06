## Context

Visualizador arranca como un proyecto nuevo. Debe integrarse con un Procesador externo que publica eventos en RabbitMQ, mantener estado propio en PostgreSQL y exponer una API con dashboard en tiempo real. El proyecto debe quedar listo para desarrollo local y para crecer sin acoplar reglas de dominio a detalles de infraestructura.

## Goals / Non-Goals

**Goals:**

- Definir un workspace Cargo con separacion clara entre dominio, adaptadores e interfaz API.
- Mantener el crate `dominio` libre de dependencias de infraestructura.
- Consumir eventos de retiro y devolucion desde RabbitMQ.
- Persistir recorridos y estado actual de bicicletas con `sqlx` y migraciones SQL.
- Publicar cambios de estado por WebSocket nativo usando `axum`.
- Proveer entorno local reproducible con Docker Compose y README de desarrollo.

**Non-Goals:**

- No implementar CLI de ingesta CSV.
- No validar existencia o consistencia de estaciones contra un catalogo externo.
- No usar Socket.IO ni protocolos no nativos sobre WebSocket.
- No implementar autenticacion, autorizacion ni frontend complejo de produccion en este change.

## Decisions

### Workspace y arquitectura hexagonal

El workspace tendra crates `dominio`, `adaptadores` y `api`. `dominio` definira entidades, eventos, errores y traits de puertos; `adaptadores` implementara PostgreSQL, RabbitMQ y broadcasting; `api` sera el binario que carga configuracion, compone dependencias y expone HTTP/WebSocket.

Alternativa considerada: un unico crate binario. Se descarta porque mezclaria reglas de negocio, infraestructura y composicion desde el inicio, haciendo mas dificil testear reglas sin servicios externos.

### Dominio como fuente de reglas

Las reglas para retiro y devolucion viviran en servicios de dominio que dependan de traits. Un retiro abrira un recorrido activo y marcara la bicicleta como retirada. Una devolucion cerrara el recorrido activo y marcara la bicicleta como disponible. Los errores de negocio usaran `thiserror` y nombres en espanol.

Alternativa considerada: implementar las reglas directamente en el consumidor RabbitMQ. Se descarta porque acoplaria reglas a transporte y complicaria pruebas unitarias.

### Persistencia PostgreSQL con sqlx

Las migraciones iniciales crearan `recorridos` y `estado_bicicletas`. El adaptador PostgreSQL implementara puertos transaccionales para registrar eventos, consultar recorridos activos y actualizar estado. `sqlx` se usara con tipos Rust explicitos y pool compartido.

Alternativa considerada: usar ORM de mayor nivel. Se prefiere `sqlx` por consultas SQL explicitas, migraciones simples y buena integracion async.

### Consumo RabbitMQ con lapin

El adaptador RabbitMQ consumira la cola `bike_trips` y deserializara mensajes JSON de retiro/devolucion. El ack del mensaje ocurrira despues de persistir exitosamente y publicar la actualizacion interna; ante error procesable, el mensaje podra rechazarse o reencolarse segun el tipo de error.

Alternativa considerada: polling HTTP contra el Procesador. Se descarta porque el contrato indicado es RabbitMQ.

### Dashboard WebSocket con axum

La API expondra un endpoint WebSocket nativo. Cada cliente conectado recibira actualizaciones serializadas cuando cambie el estado de bicicletas o recorridos. Para el primer corte, un canal broadcast en memoria es suficiente; PostgreSQL sigue siendo la fuente de verdad.

Alternativa considerada: Socket.IO. Se excluye explicitamente del alcance para mantener protocolo WebSocket nativo.

## Risks / Trade-offs

- Perdida de mensajes si el proceso cae entre persistir y emitir al broadcast -> PostgreSQL conserva la fuente de verdad y el dashboard podra consultar snapshot inicial antes de recibir eventos en vivo.
- Duplicados de RabbitMQ por reentregas -> las operaciones de persistencia deben ser idempotentes cuando exista identificador de evento o recorrido.
- Contrato de mensaje externo incompleto -> documentar el formato esperado en README y aislar la deserializacion en adaptadores.
- Broadcast en memoria no escala entre multiples replicas -> aceptable para el bootstrap; una evolucion futura puede usar Redis, Postgres LISTEN/NOTIFY o fanout por broker.
- Tests con servicios externos pueden ser mas lentos -> concentrar reglas en tests unitarios de `dominio` y dejar integracion para adaptadores criticos.

## Migration Plan

1. Crear estructura del workspace y crates.
2. Agregar migraciones SQL y configurar `sqlx`.
3. Implementar dominio y puertos con tests unitarios.
4. Implementar adaptadores PostgreSQL, RabbitMQ y broadcast.
5. Componer el binario `api` con configuracion por variables de entorno.
6. Agregar `docker-compose.yml` y README de desarrollo.
7. Verificar `cargo build` y `cargo test`.

Rollback: al ser proyecto nuevo, revertir el change remueve el workspace, migraciones y documentacion agregados.

## Open Questions

- El formato exacto del mensaje RabbitMQ debe confirmarse con el servicio Procesador; se asumira JSON con tipo de evento, bicicleta, estaciones y timestamps.
- La estrategia final para idempotencia depende de si los eventos externos incluyen un identificador estable.
