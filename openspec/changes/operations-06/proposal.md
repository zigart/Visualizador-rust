## Why

Visualizador necesita endpoints operativos para health checks, publicacion manual de mensajes, logging estructurado y artefactos minimos de despliegue. Esto permite probar flujos sin publicar manualmente desde la UI de RabbitMQ y preparar el servicio para ejecucion containerizada.

## What Changes

- Agregar `GET /health` con version del paquete.
- Agregar `POST /rabbit/mensajes` para publicar mensajes JSON a RabbitMQ.
- Proteger publicacion manual con headers `X-Usuario` y `X-Contrasena`.
- Mejorar logs de inicio/fin de procesamiento y errores de dominio con campos estructurados.
- Soportar `LOG_FORMAT=json` para produccion.
- Agregar Dockerfile multi-stage.
- Agregar manifests Kubernetes con `livenessProbe` en `/health`.

## Capabilities

### New Capabilities

- `operations`: Health check, publicacion manual, logging estructurado y despliegue containerizado.

### Modified Capabilities

- None.

## Impact

- Actualiza `crates/api` con handlers operativos y middleware/auth simple por headers.
- Actualiza adaptadores RabbitMQ con publicacion manual si hace falta.
- Agrega `Dockerfile` e `infra/k8s/*`.
