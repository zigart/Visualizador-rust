# Visualizador Rust

Servicio Rust para consumir eventos de retiros/devoluciones de bicicletas, persistir estado en PostgreSQL y publicar actualizaciones en tiempo real por WebSocket.

Este bootstrap deja listo el workspace, el entorno local y las migraciones iniciales. La logica de negocio y los adaptadores concretos se implementan en pasos posteriores del change.

## Requisitos

- Rust estable con `cargo`
- Docker Desktop o Docker Engine con Compose v2
- `sqlx-cli` con soporte PostgreSQL:

```powershell
cargo install sqlx-cli --no-default-features --features postgres
```

## Instalacion equivalente a `yarn install`

En Rust no hay un paso separado de instalacion como `yarn install`: Cargo descarga y compila dependencias al ejecutar build o test.

```powershell
cargo fetch
cargo build
```

## Entorno local

Levantar PostgreSQL y RabbitMQ:

```powershell
docker compose up -d
```

Servicios expuestos:

- PostgreSQL: `localhost:5432`
- RabbitMQ AMQP: `localhost:5672`
- RabbitMQ Management: `http://localhost:15672`

Credenciales por defecto:

- PostgreSQL user: `visualizador`
- PostgreSQL password: `visualizador`
- PostgreSQL database: `visualizador`
- RabbitMQ user: `visualizador`
- RabbitMQ password: `visualizador`

## Variables de entorno

```powershell
$env:DATABASE_URL = "postgres://visualizador:visualizador@localhost:5432/visualizador"
$env:RABBITMQ_URL = "amqp://visualizador:visualizador@localhost:5672/%2f"
$env:QUEUE_NAME = "bike_trips"
$env:RABBIT_PREFETCH = "1"
$env:OPERATOR_USER = "visualizador"
$env:OPERATOR_PASSWORD = "visualizador"
$env:HTTP_BIND = "127.0.0.1:3000"
$env:APP_ENV = "dev"
$env:LOG_FORMAT = "pretty"
```

Tambien se puede copiar `.env.example` a `.env` para desarrollo local.

## Migraciones

Con PostgreSQL levantado:

```powershell
sqlx migrate run
```

La migracion inicial crea:

- `recorridos`
- `estado_bicicletas`, con fila singleton `id = 1` y contadores en cero

## Formato esperado de mensajes RabbitMQ

La cola por defecto es `bike_trips`, configurable con `QUEUE_NAME`. El consumer usa `RABBIT_PREFETCH` para limitar mensajes sin confirmar. El contrato esperado para eventos JSON es:

```json
{
  "id_recorrido": 1,
  "id_usuario": 1,
  "id_estacion": 10,
  "operacion": "retiro",
  "fechahora": "2026-06-08T15:34:20Z"
}
```

`operacion` acepta `retiro` o `devolucion`. `id_recorrido`, `id_usuario` e `id_estacion` pueden llegar como numero o string numerico. `id_estacion` es opcional hasta cerrar el contrato final con Procesador.

## Endpoints operativos

Health check:

```powershell
Invoke-RestMethod http://127.0.0.1:3000/health
```

Publicacion manual a RabbitMQ:

```powershell
Invoke-RestMethod `
  -Uri "http://127.0.0.1:3000/rabbit/mensajes" `
  -Method Post `
  -Headers @{ "X-Usuario" = "visualizador"; "X-Contrasena" = "visualizador" } `
  -ContentType "application/json" `
  -Body '{"id_recorrido":1,"id_usuario":1,"id_estacion":10,"operacion":"retiro","fechahora":"2026-06-08T15:34:20Z"}'
```

## Imagen Docker

```powershell
docker build -t visualizador-rust:local .
docker run --rm -p 3000:3000 --env-file .env visualizador-rust:local
```

## Kubernetes

Los manifests base estan en `infra/k8s/`:

```powershell
kubectl apply -f infra/k8s/
```

Configurar las variables `DATABASE_URL`, `RABBITMQ_URL`, `QUEUE_NAME`, `RABBIT_PREFETCH`, `OPERATOR_USER`, `OPERATOR_PASSWORD`, `HTTP_BIND` y `LOG_FORMAT` segun el entorno.

## Comandos de desarrollo

```powershell
cargo fmt
cargo build
cargo test
```
