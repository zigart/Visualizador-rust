## Why

El contrato con Procesador quedó definido: todo movimiento RabbitMQ debe incluir `id_estacion`. Hoy el parser lo trata como opcional (`null`, ausente o string vacío se aceptan), lo que permite persistir recorridos sin estación y rompe la reconstrucción de viajes en `/usuarios`.

## What Changes

- **BREAKING**: `id_estacion` pasa a ser campo obligatorio en el mensaje JSON, al mismo nivel que `id_recorrido` e `id_usuario`.
- Rechazar mensajes sin `id_estacion`, con valor `null`, string vacío o no numérico (mismo criterio que otros IDs obligatorios).
- Actualizar el modelo de dominio `MovimientoRecorrido` para representar `id_estacion` como `u64` no opcional.
- Ajustar persistencia, tests y documentación que asumían `id_estacion` opcional.
- Migración SQL para `NOT NULL` en `recorridos.id_estacion` (con backfill o limpieza de filas huérfanas si aplica).

## Capabilities

### New Capabilities

_(ninguna — el cambio extiende capacidades existentes)_

### Modified Capabilities

- `message-validation`: `id_estacion` obligatorio; rechazo ante ausencia, `null`, vacío o valor inválido.
- `persistence`: columna `id_estacion` NOT NULL; INSERT siempre con valor.
- `usuario-viajes`: escenarios que asumían estación opcional deben reflejar obligatoriedad en mensaje entrante.

## Impact

- `crates/adaptadores/src/validacion_mensajes.rs` — parser y tests.
- `crates/dominio/src/mensajes.rs` — tipo `MovimientoRecorrido`.
- `crates/adaptadores/src/postgres/recorrido.rs` — INSERT/SELECT sin `Option`.
- `crates/dominio/src/sistema.rs`, tests de integración y smoke scripts.
- `migrations/` — nueva migración `NOT NULL`.
- `README.md` — contrato de mensaje actualizado.
