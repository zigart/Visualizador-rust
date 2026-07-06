## 1. Modelo de dashboard

- [x] 1.1 Definir `EstadoDashboard` serializable con campos requeridos.
- [x] 1.2 Definir modelo opcional de movimiento para el payload.
- [x] 1.3 Definir trait `VistaEstadoBicicletas` para consultar estado actual.

## 2. Axum routes

- [x] 2.1 Crear router de dashboard en `crates/api`.
- [x] 2.2 Implementar `GET /dashboard` sirviendo `static/dashboard/index.html`.
- [x] 2.3 Servir assets CSS/JS bajo una ruta estatica.
- [x] 2.4 Registrar rutas en `main`.

## 3. WebSocket handler

- [x] 3.1 Implementar endpoint `/ws` con WebSocket nativo de `axum`.
- [x] 3.2 Al conectar, consultar `VistaEstadoBicicletas` y enviar estado inicial.
- [x] 3.3 Suscribir cada cliente a `tokio::sync::broadcast::Sender<EstadoDashboard>`.
- [x] 3.4 Enviar cada actualizacion broadcast como JSON por WebSocket.
- [x] 3.5 Manejar desconexion de cliente sin afectar otros clientes.

## 4. Broadcast tras movimiento

- [x] 4.1 Integrar envio de `EstadoDashboard` despues de cada movimiento valido.
- [x] 4.2 Incluir movimiento opcional en payload cuando exista evento.
- [x] 4.3 Loguear errores de broadcast sin fallar el procesamiento principal.

## 5. UI estatica

- [x] 5.1 Crear `static/dashboard/index.html`.
- [x] 5.2 Crear `static/dashboard/styles.css`.
- [x] 5.3 Crear `static/dashboard/app.js`.
- [x] 5.4 Mostrar disponibles, en uso, ultima actualizacion en formato `es-AR` y estado de conexion.
- [x] 5.5 Agregar movimientos recibidos al historial visible sin recargar.

## 6. Tests y verificacion

- [x] 6.1 Agregar tests para serializacion de `EstadoDashboard`.
- [x] 6.2 Agregar tests para `GET /dashboard`.
- [x] 6.3 Agregar tests para envio de estado inicial WebSocket si es viable.
- [x] 6.4 Ejecutar `cargo fmt`.
- [x] 6.5 Ejecutar `cargo build`.
- [x] 6.6 Ejecutar `cargo test`.
- [x] 6.7 Validar el change con `openspec validate`.
