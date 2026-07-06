## 1. Dominio y validacion de mensajes

- [x] 1.1 Cambiar `MovimientoRecorrido.id_estacion` de `Option<u64>` a `u64` en `crates/dominio/src/mensajes.rs`.
- [x] 1.2 Usar `parse_required_id` para `id_estacion` en `validacion_mensajes.rs`.
- [x] 1.3 Agregar tests: ausente, `null`, string vacío, no numérico y string numérico válido.
- [x] 1.4 Actualizar tests existentes que omiten `id_estacion` en payloads válidos.

## 2. Persistencia y migracion

- [x] 2.1 Crear migracion: eliminar filas con `id_estacion IS NULL` y `ALTER COLUMN id_estacion SET NOT NULL`.
- [x] 2.2 Simplificar `recorrido.rs` (INSERT/mapeo sin `Option` para `id_estacion`).
- [x] 2.3 Actualizar `sistema.rs` y helpers de test que construyen `MovimientoRecorrido`.

## 3. Tests de integracion y documentacion

- [x] 3.1 Actualizar `persistencia_postgres.rs` y asserts de esquema (`NOT NULL`).
- [x] 3.2 Verificar smoke script y README: `id_estacion` obligatorio en contrato JSON.

## 4. Verificacion

- [x] 4.1 Ejecutar `cargo fmt`.
- [x] 4.2 Ejecutar `cargo build`.
- [x] 4.3 Ejecutar `cargo test`.
- [x] 4.4 Validar el change con `openspec validate require-id-estacion-10`.
