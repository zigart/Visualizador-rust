## Why

Visualizador ya parsea movimientos entrantes, pero todavia no conserva esos movimientos ni el estado agregado en PostgreSQL. Este change agrega repositorios `sqlx` para persistir movimientos aceptados, actualizar el estado singleton y consultar movimientos vecinos por usuario y fecha.

## What Changes

- Implementar `RepositorioRecorrido` con `sqlx` para insertar y consultar movimientos.
- Implementar `RepositorioEstadoBicicletas` con `sqlx` para actualizar la fila singleton `estado_bicicletas.id = 1`.
- Persistir movimiento y estado en una misma transaccion.
- Agregar consultas temporales para obtener ultimo y siguiente movimiento por usuario y fecha.
- Ajustar migraciones si el esquema actual no contiene los campos requeridos por el contrato de movimientos.
- Agregar tests de integracion con `testcontainers`.

## Capabilities

### New Capabilities

- `persistence`: Repositorios PostgreSQL con `sqlx` para movimientos, estado agregado, consultas temporales y transaccionalidad.

### Modified Capabilities

- None.

## Impact

- Actualiza migraciones SQL para soportar `id_recorrido`, `id_usuario`, `operacion`, `fechahora`, `en_uso` y `maximo_historico`.
- Agrega implementaciones PostgreSQL en `crates/adaptadores`.
- Agrega tests de integracion que levantan PostgreSQL con `testcontainers`.
