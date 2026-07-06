## 1. Health check

- [ ] 1.1 Crear handler `GET /health`.
- [ ] 1.2 Retornar JSON con `version = env!("CARGO_PKG_VERSION")`.
- [ ] 1.3 Registrar ruta en el router `axum`.
- [ ] 1.4 Agregar test de respuesta 200 y version.

## 2. Publicacion manual RabbitMQ

- [ ] 2.1 Agregar configuracion de credenciales operativas para `X-Usuario` y `X-Contrasena`.
- [ ] 2.2 Implementar middleware o extractor de auth por headers.
- [ ] 2.3 Agregar publicador RabbitMQ con `lapin`.
- [ ] 2.4 Crear handler `POST /rabbit/mensajes`.
- [ ] 2.5 Publicar payload al exchange/routing key configurado.
- [ ] 2.6 Responder `202 Accepted` tras publicar.
- [ ] 2.7 Agregar tests para auth valida y faltante.

## 3. Logging estructurado

- [ ] 3.1 Loguear inicio de procesamiento con `id_recorrido`, `id_usuario` y `operacion`.
- [ ] 3.2 Loguear fin de procesamiento exitoso.
- [ ] 3.3 Loguear errores de dominio con campos estructurados y mensaje de error.
- [ ] 3.4 Verificar que `LOG_FORMAT=json` inicializa subscriber JSON.

## 4. Dockerfile

- [ ] 4.1 Crear Dockerfile multi-stage.
- [ ] 4.2 Compilar binario `api` en stage builder.
- [ ] 4.3 Copiar binario y assets estaticos al stage runtime.
- [ ] 4.4 Documentar build/run de imagen.

## 5. Kubernetes

- [ ] 5.1 Crear `infra/k8s/deployment.yaml`.
- [ ] 5.2 Agregar `livenessProbe` HTTP en `/health`.
- [ ] 5.3 Crear `infra/k8s/service.yaml`.
- [ ] 5.4 Documentar variables de entorno requeridas para K8s.

## 6. Verificacion

- [ ] 6.1 Ejecutar `cargo fmt`.
- [ ] 6.2 Ejecutar `cargo build`.
- [ ] 6.3 Ejecutar `cargo test`.
- [ ] 6.4 Validar el change con `openspec validate`.
