## Why

El consumer RabbitMQ necesita validar el contrato de mensajes antes de ejecutar reglas de negocio. Rechazar payloads invalidos temprano evita estados inconsistentes y permite responder al broker con ACK/NACK de forma predecible.

## What Changes

- Agregar parseo de payloads JSON entrantes con objeto raiz obligatorio.
- Validar campos obligatorios `id_recorrido`, `id_usuario`, `operacion` y `fechahora`.
- Aceptar solo operaciones `retiro` y `devolucion`.
- Validar `fechahora` como fecha ISO 8601 parseable.
- Coercionar `id_recorrido` e `id_usuario` cuando llegan como string numerico.
- Mapear errores de validacion a errores de dominio.

## Capabilities

### New Capabilities

- `message-validation`: Parseo, coercion y validacion de mensajes JSON entrantes desde RabbitMQ.

### Modified Capabilities

- None.

## Impact

- Agrega DTOs y parser de mensajes entrantes.
- Extiende errores de dominio para campos obligatorios y valores invalidos.
- Alimenta el flujo ACK/NACK del consumer con errores de validacion claros.
