## Why

Visualizador necesita exponer una consulta de historial por usuario para que consumidores externos puedan revisar viajes completados y viajes en curso. El endpoint requiere agrupar retiros y devoluciones por `id_recorrido`, ordenar por devolucion y conservar informacion de estaciones.

## What Changes

- Agregar endpoint `GET /usuarios/{id_usuario}`.
- Retornar coleccion `viajes` del usuario.
- Ordenar viajes por `fechaHoraDevolucion` ascendente con pendientes al final.
- Representar viajes en curso con campos de devolucion en `null`.
- Persistir y aceptar `id_estacion` en mensajes/movimientos.
- Agregar consulta `listar_viajes_por_usuario` en repositorio de recorridos.

## Capabilities

### New Capabilities

- `usuario-viajes`: Consulta HTTP de historial de viajes por usuario.

### Modified Capabilities

- `persistence`: Persistir `id_estacion` para reconstruir estaciones de retiro/devolucion.
- `message-validation`: Aceptar y validar `id_estacion` del mensaje entrante segun contrato acordado.

## Impact

- Nueva ruta HTTP en `crates/api`.
- Nueva consulta de repositorio en adaptadores PostgreSQL.
- Migracion para agregar `id_estacion` a `recorridos`.
- Ajuste de DTO/parser de mensajes si el contrato incluye estaciones.
- Tests de aceptacion para usuario encontrado, orden y 404.
