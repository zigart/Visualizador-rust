## Context

El servicio ya tiene base para API `axum`, consumer RabbitMQ y dominio. Para visualizar el estado, se necesita una UI liviana que use WebSocket nativo y reciba snapshots/actualizaciones de `EstadoDashboard`.

## Goals / Non-Goals

**Goals:**

- Servir HTML/CSS/JS desde `GET /dashboard`.
- Exponer WebSocket `/ws` con `axum`.
- Enviar estado persistido al conectar.
- Broadcast de actualizaciones tras movimientos validos.
- Mantener la lectura de estado detras del trait `VistaEstadoBicicletas`.

**Non-Goals:**

- No usar Socket.IO.
- No construir SPA con framework frontend.
- No implementar autenticacion ni autorizacion.

## Decisions

### Assets estaticos

Los archivos viviran bajo `static/dashboard/` y seran servidos por la API. `index.html` incluira indicadores principales y una lista de eventos; `app.js` abrira `/ws` y actualizara el DOM.

### EstadoDashboard

El payload serializable incluira:

- `bicicletas_disponibles`
- `bicicletas_en_uso`
- `actualizado_en`
- `estado_conexion`
- `movimiento`

`movimiento` sera opcional para permitir snapshot inicial sin evento asociado.

### WebSocket y broadcast

La API mantendra `tokio::sync::broadcast::Sender<EstadoDashboard>`. Cada conexion `/ws` tendra un receiver propio. Al conectar, primero se consultara `VistaEstadoBicicletas` y se enviara el estado actual; luego el cliente recibira actualizaciones del broadcast.

### Trait VistaEstadoBicicletas

`VistaEstadoBicicletas` abstraera la lectura de estado persistido para que la ruta WebSocket no dependa directamente de PostgreSQL. La implementacion concreta puede vivir en adaptadores.

## Risks / Trade-offs

- Broadcast en memoria no distribuye entre multiples replicas -> aceptable para el primer dashboard.
- Si el estado inicial falla, el WebSocket debe cerrar o enviar error controlado -> preferir logs y cierre limpio.
- UI sin framework limita complejidad visual -> suficiente para indicadores e historial operativo.
