## 1. Health check

- [x] 1.1 Crear handler `GET /health`.
- [x] 1.2 Retornar JSON con `version = env!("CARGO_PKG_VERSION")`.
- [x] 1.3 Registrar ruta en el router `axum`.
- [x] 1.4 Agregar test de respuesta 200 y version.

## 2. Publicacion manual RabbitMQ

- [x] 2.1 Agregar configuracion de credenciales operativas para `X-Usuario` y `X-Contrasena`.
- [x] 2.2 Implementar middleware o extractor de auth por headers.
- [x] 2.3 Agregar publicador RabbitMQ con `lapin`.
- [x] 2.4 Crear handler `POST /rabbit/mensajes`.
- [x] 2.5 Publicar payload al exchange/routing key configurado.
- [x] 2.6 Responder `202 Accepted` tras publicar.
- [x] 2.7 Agregar tests para auth valida y faltante.

## 3. Logging estructurado

- [x] 3.1 Loguear inicio de procesamiento con `id_recorrido`, `id_usuario` y `operacion`.
- [x] 3.2 Loguear fin de procesamiento exitoso.
- [x] 3.3 Loguear errores de dominio con campos estructurados y mensaje de error.
- [x] 3.4 Verificar que `LOG_FORMAT=json` inicializa subscriber JSON.

## 4. Dockerfile

- [x] 4.1 Crear Dockerfile multi-stage.
- [x] 4.2 Compilar binario `api` en stage builder.
- [x] 4.3 Copiar binario y assets estaticos al stage runtime.
- [x] 4.4 Documentar build/run de imagen.

## 5. Kubernetes

- [x] 5.1 Crear `infra/k8s/deployment.yaml`.
- [x] 5.2 Agregar `livenessProbe` HTTP en `/health`.
- [x] 5.3 Crear `infra/k8s/service.yaml`.
- [x] 5.4 Documentar variables de entorno requeridas para K8s.

## 6. Verificacion

- [x] 6.1 Ejecutar `cargo fmt`.
- [x] 6.2 Ejecutar `cargo build`.
- [x] 6.3 Ejecutar `cargo test`.
- [x] 6.4 Validar el change con `openspec validate`.
- [x] 6.5 Ejecutar smoke e2e: publicar mensaje y observar actualizacion WebSocket.
