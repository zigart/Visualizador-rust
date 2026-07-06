## 1. Migracion SQL

- [x] 1.1 Crear migracion incremental que recrea `recorridos` con esquema objetivo (SERIAL, INTEGER, VARCHAR(20), TIMESTAMPTZ, BIGINT NULL).
- [x] 1.2 Recrear `estado_bicicletas` sin `updated_at`, sin CHECK singleton y sin checks de no-negativo.
- [x] 1.3 Migrar datos existentes mapeando solo columnas del contrato; descartar columnas legacy.
- [x] 1.4 Eliminar indices obsoletos (`recorridos_bicicleta_activa_idx`) y recrear indices de consulta temporal.
- [x] 1.5 Verificar `sqlx migrate run` en entorno limpio y con datos de prueba.

## 2. Adaptador recorrido

- [x] 2.1 Simplificar `insertar_movimiento` en `recorrido.rs` a INSERT de 5 columnas del contrato.
- [x] 2.2 Ajustar bindings de tipos (`i32` para INTEGER) si corresponde.
- [x] 2.3 Verificar que consultas SELECT temporales siguen funcionando sin columnas legacy.

## 3. Adaptador estado

- [x] 3.1 Quitar `updated_at = NOW()` del UPDATE en `estado.rs`.
- [x] 3.2 Confirmar lectura de estado con seed `id = 1` o estrategia acordada en design.

## 4. Tests y documentacion

- [x] 4.1 Actualizar `persistencia_postgres.rs` para validar esquema objetivo.
- [x] 4.2 Agregar test que confirma ausencia de columnas legacy (opcional via information_schema).
- [x] 4.3 Documentar migracion y esquema objetivo en README.

## 5. Verificacion

- [x] 5.1 Ejecutar `cargo fmt`.
- [x] 5.2 Ejecutar `cargo build`.
- [x] 5.3 Ejecutar `cargo test`.
- [x] 5.4 Validar el change con `openspec validate`.
