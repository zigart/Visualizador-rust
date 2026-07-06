## Context

El servicio ya expone dashboard y consume mensajes desde RabbitMQ. Para operar y probar el sistema hacen falta endpoints de liveness, una forma controlada de publicar mensajes al broker y artefactos de despliegue reproducibles.

## Goals / Non-Goals

**Goals:**

- Exponer `GET /health` con version del binario.
- Exponer `POST /rabbit/mensajes` que publica mensajes al exchange de RabbitMQ y responde `202`.
- Validar headers `X-Usuario` y `X-Contrasena` antes de publicar manualmente.
- Estandarizar logs con `tracing`.
- Permitir `LOG_FORMAT=json`.
- Agregar Dockerfile multi-stage y manifests K8s.

**Non-Goals:**

- No reemplazar autenticacion productiva; los headers son una proteccion operativa simple.
- No implementar autorizacion granular por rol.
- No agregar charts Helm.

## Decisions

### Health check

`GET /health` retornara JSON con `version` usando `env!("CARGO_PKG_VERSION")`. Debe ser rapido y no depender de RabbitMQ ni PostgreSQL para funcionar como liveness probe.

### Publicacion manual

`POST /rabbit/mensajes` recibira el JSON de recorrido y lo publicara al exchange default usando routing key configurable o la cola configurada. Si faltan o no coinciden `X-Usuario` y `X-Contrasena`, respondera `401`.

### Logging estructurado

El consumer y handlers loguearan inicio/fin de procesamiento. Ante errores de dominio, `tracing::error` incluira `id_recorrido`, `id_usuario`, `operacion` y mensaje de error cuando esos campos esten disponibles.

### Logging JSON

`LOG_FORMAT=json` inicializara `tracing-subscriber` en formato JSON; cualquier otro valor usara pretty/dev.

### Despliegue

El Dockerfile sera multi-stage: build con imagen Rust y runtime minimo. Los manifests en `infra/k8s/` incluiran Deployment, Service y `livenessProbe` HTTP contra `/health`.

## Risks / Trade-offs

- Auth por headers no es suficiente para produccion expuesta a internet -> usar solo en entornos controlados o reemplazar por auth real luego.
- Health sin dependencia externa puede no detectar caida de RabbitMQ/PostgreSQL -> es intencional para liveness; readiness puede agregarse despues.
- Publicacion manual duplica una funcion disponible en RabbitMQ Management -> mejora ergonomia local y permite pruebas automatizadas.
