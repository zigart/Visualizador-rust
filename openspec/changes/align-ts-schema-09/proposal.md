## Why

El esquema PostgreSQL actual de visualizador-rust conserva columnas legacy del bootstrap (`bicicleta_id`, `iniciado_en`, `finalizado_en`, etc.) y constraints de `estado_bicicletas` que no forman parte del contrato objetivo alineado con el servicio TypeScript. Los adaptadores `sqlx` rellenan esas columnas derivadas solo para satisfacer `NOT NULL`, lo que complica mantenimiento y diverge del modelo de dominio.

## What Changes

- Agregar migracion incremental que elimina columnas legacy de `recorridos` y `estado_bicicletas`.
- Alinear tipos de columnas al contrato objetivo (`SERIAL`, `INTEGER`, `VARCHAR(20)`, `TIMESTAMPTZ`, `BIGINT`).
- Eliminar indices y constraints obsoletos (`recorridos_bicicleta_activa_idx`, singleton `id=1`, checks de no-negativo, `updated_at`).
- Simplificar `INSERT` en `recorrido.rs` para usar solo columnas del contrato.
- Simplificar `UPDATE` en `estado.rs` sin `updated_at`.
- Actualizar tests de integracion de persistencia para el nuevo esquema.
- Documentar esquema objetivo en delta spec `persistence`.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `persistence`: esquema de tablas `recorridos` y `estado_bicicletas` y operaciones SQL de adaptadores.

## Impact

- Nueva migracion SQL en `migrations/`.
- Cambios en `crates/adaptadores/src/postgres/recorrido.rs` y `estado.rs`.
- Posible ajuste de lectura de `estado_bicicletas` (sin filtro singleton `id=1` fijo si el contrato usa `SERIAL` generico).
- Tests en `crates/adaptadores/tests/persistencia_postgres.rs`.
- README: nota de migracion para entornos con datos legacy.
