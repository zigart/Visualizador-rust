## Why

Visualizador necesita una superficie simple para observar el estado actual y los movimientos procesados sin depender de herramientas externas. Este change agrega un dashboard estatico servido por la API y un WebSocket nativo para actualizaciones en tiempo real.

## What Changes

- Servir dashboard estatico en `GET /dashboard`.
- Exponer WebSocket nativo en `/ws`.
- Enviar `EstadoDashboard` al conectar usando estado persistido.
- Emitir `EstadoDashboard` a todos los clientes tras cada movimiento valido.
- Agregar payload JSON con indicadores, estado de conexion y movimiento opcional.
- Agregar UI con indicadores y lista visible de eventos sin recarga.
- Introducir trait `VistaEstadoBicicletas` para consultar estado actual sin acoplar dashboard al repositorio concreto.

## Capabilities

### New Capabilities

- `dashboard`: Dashboard estatico y WebSocket de estado en tiempo real.

### Modified Capabilities

- None.

## Impact

- Agrega rutas `axum` para `/dashboard` y `/ws`.
- Agrega assets en `static/dashboard/*`.
- Agrega canal broadcast para fan-out de `EstadoDashboard`.
- Agrega trait de lectura `VistaEstadoBicicletas` para cargar estado inicial.
