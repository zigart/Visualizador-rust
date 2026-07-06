## Context

Visualizador arranca como un proyecto Rust nuevo. Debe consumir eventos de retiro y devolucion desde RabbitMQ, aplicar reglas de negocio, persistir recorridos y estado actual en PostgreSQL, y publicar cambios en tiempo real por WebSocket nativo.

La arquitectura debe mantener el dominio aislado de infraestructura para que las reglas se puedan probar sin I/O y para que los adaptadores evolucionen sin contaminar el modelo central.

## Goals / Non-Goals

**Goals:**

- Definir un workspace Cargo con crates `crates/dominio`, `crates/adaptadores` y `crates/api`.
- Ejecutar el consumidor RabbitMQ y el servidor HTTP/WebSocket sobre `tokio` multi-thread.
- Mantener errores de dominio tipados con `thiserror` y errores de infraestructura con `anyhow`.
- Cargar configuracion desde variables de entorno usando `config` y `dotenvy`.
- Compartir un `sqlx::PgPool` entre componentes mediante `Arc`.
- Emitir fan-out de actualizaciones de dashboard con `tokio::sync::broadcast`.
- Cubrir reglas con tests unitarios sin I/O e integraciones con `testcontainers`.

**Non-Goals:**

- No implementar CLI de ingesta CSV.
- No validar estaciones contra un catalogo externo.
- No usar Socket.IO ni protocolos no nativos sobre WebSocket.
- No implementar autenticacion, autorizacion ni frontend complejo de produccion en este change.

## Module Diagram

```text
visualizador-rust/
├── Cargo.toml
├── migrations/
└── crates/
    ├── dominio/
    │   ├── entidades
    │   ├── servicios
    │   ├── puertos
    │   └── ErrorDominio
    ├── adaptadores/
    │   ├── postgres
    │   ├── rabbitmq
    │   └── dashboard_broadcast
    └── api/
        ├── AppConfig
        ├── http_axum
        ├── websocket
        └── main
```

`dominio` no depende de `adaptadores` ni de `api`. `adaptadores` depende de `dominio` para implementar puertos. `api` compone configuracion, pool PostgreSQL, consumidor RabbitMQ, broadcast y rutas HTTP/WebSocket.

## Data Flow

```text
RabbitMQ
   |
   v
adaptadores::rabbitmq
   |
   v
dominio::servicios
   |
   v
adaptadores::postgres -> PostgreSQL
   |
   v
tokio::sync::broadcast::Sender<EstadoDashboard>
   |
   v
api::websocket -> clientes WebSocket
```

El consumidor confirma mensajes RabbitMQ solo despues de que el dominio acepta la operacion, PostgreSQL persiste el cambio y se emite la actualizacion al canal broadcast.

## Decisions

### 1. Workspace layout

El workspace tendra:

- `crates/dominio`: entidades, value objects, reglas, puertos y `ErrorDominio`.
- `crates/adaptadores`: implementaciones PostgreSQL, RabbitMQ y broadcast.
- `crates/api`: binario `axum`, carga de configuracion y composicion de dependencias.

Alternativa considerada: un crate unico. Se descarta porque mezclaria reglas, transporte y persistencia desde el inicio.

### 2. Runtime

`api` usara `#[tokio::main(flavor = "multi_thread")]`. El servidor HTTP/WebSocket y el consumidor RabbitMQ se ejecutaran como tasks paralelas supervisadas desde `main`.

Alternativa considerada: ejecutar consumidor y servidor en procesos separados. Se deja fuera del bootstrap para reducir operacion inicial, manteniendo separacion interna por modulos.

### 3. Errores

`dominio` definira `enum ErrorDominio` con `thiserror` para errores de negocio como retiro duplicado o devolucion sin recorrido activo. `adaptadores` usara `anyhow::Result` para envolver errores de I/O, serializacion, RabbitMQ y PostgreSQL, convirtiendo errores de dominio cuando corresponda.

Alternativa considerada: usar `anyhow` en todo el proyecto. Se descarta para dominio porque perderia exhaustividad y claridad de reglas.

### 4. Config

`api` definira `AppConfig` cargada desde variables de entorno con `config` y soporte local de `.env` mediante `dotenvy`. La configuracion incluira URLs de PostgreSQL y RabbitMQ, cola RabbitMQ, prefetch, bind HTTP, formato de logs y entorno.

Alternativa considerada: leer variables directamente en cada modulo. Se descarta porque dispersa defaults y dificulta tests.

### 5. DB

PostgreSQL se accedera con `sqlx::PgPool`, compartido entre adaptadores mediante `Arc<PgPool>`. Las migraciones viviran en `/migrations` y crearan `recorridos` y `estado_bicicletas`.

Alternativa considerada: crear pools por componente. Se descarta porque aumenta conexiones y complica transacciones coordinadas.

### 6. WebSocket fan-out

El fan-out en memoria usara `tokio::sync::broadcast::Sender<EstadoDashboard>`. Cada conexion WebSocket creara un receiver propio y enviara a su cliente los cambios serializados.

Alternativa considerada: fan-out persistente o distribuido. Se deja fuera porque el bootstrap apunta a una instancia local; PostgreSQL conserva la fuente de verdad.

### 7. RabbitMQ

El consumidor usara `lapin` con recuperacion automatica habilitada (`enable_auto_recover`) y `basic_qos` con prefetch configurable. El default de cola sera `bike_trips`.

Alternativa considerada: auto-ack. Se descarta porque puede perder mensajes si el proceso falla antes de persistir.

### 8. Logging

El proyecto usara `tracing` y `tracing-subscriber`. En desarrollo el formato sera pretty; en produccion sera JSON para facilitar recoleccion por plataformas de logs.

Alternativa considerada: `log` basico. Se descarta porque `tracing` modela mejor spans async para HTTP, RabbitMQ y DB.

### 9. Tests

`dominio` tendra tests unitarios sin I/O para retiro valido, retiro duplicado, devolucion valida y devolucion sin recorrido activo. `adaptadores` y `api` tendran tests de integracion con `testcontainers` para PostgreSQL y RabbitMQ cuando el entorno lo permita.

Alternativa considerada: probar todo con servicios reales locales. Se descarta porque vuelve la suite menos reproducible en CI y en equipos nuevos.

## Risks / Trade-offs

- Broadcast en memoria no distribuye eventos entre multiples replicas -> aceptable para bootstrap; una evolucion futura puede usar Redis, Postgres LISTEN/NOTIFY o fanout por broker.
- `anyhow` en adaptadores puede ocultar clasificacion fina de errores -> mitigar agregando contexto con `.context(...)` y preservando `ErrorDominio` cuando cruce capas.
- `testcontainers` agrega costo a tests de integracion -> mantener tests unitarios rapidos en dominio y marcar integraciones claramente.
- Auto recovery de RabbitMQ no reemplaza idempotencia -> el dominio y la persistencia deben rechazar estados invalidos ante reentregas.

## Migration Plan

1. Crear workspace y crates.
2. Definir dominio, puertos y `ErrorDominio`.
3. Agregar migraciones `/migrations` y adaptador PostgreSQL con `PgPool`.
4. Agregar fan-out `broadcast::Sender<EstadoDashboard>`.
5. Agregar consumidor RabbitMQ con `lapin`, auto recover y prefetch configurable.
6. Componer runtime `tokio` multi-thread en `api`.
7. Agregar logging, configuracion y README.
8. Verificar `cargo build`, `cargo test` y `openspec validate`.

Rollback: al ser proyecto nuevo, revertir el change remueve workspace, migraciones y documentacion agregados.

## Open Questions

- El formato exacto del mensaje RabbitMQ debe confirmarse con el productor externo; se asumira JSON con tipo de evento, bicicleta, estacion y timestamp.
- La estrategia final de idempotencia depende de si los eventos externos incluyen un identificador estable.
