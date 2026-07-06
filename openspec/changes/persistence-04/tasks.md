## 1. Esquema PostgreSQL

- [x] 1.1 Revisar la migracion actual y definir cambios necesarios para `id_recorrido`, `id_usuario`, `operacion`, `fechahora`, `en_uso` y `maximo_historico`.
- [x] 1.2 Crear migracion incremental o ajustar bootstrap si corresponde al flujo del repo.
- [x] 1.3 Verificar que `sqlx migrate run` deja `recorridos` y `estado_bicicletas` con columnas requeridas.

## 2. RepositorioRecorrido

- [x] 2.1 Definir tipo de fila o modelo persistido para movimientos.
- [x] 2.2 Implementar INSERT de movimiento aceptado en `recorridos`.
- [x] 2.3 Implementar consulta de ultimo movimiento por usuario antes de una fecha.
- [x] 2.4 Implementar consulta de siguiente movimiento por usuario despues de una fecha.

## 3. RepositorioEstadoBicicletas

- [x] 3.1 Definir modelo de estado agregado con `en_uso` y `maximo_historico`.
- [x] 3.2 Implementar lectura de la fila singleton `id=1`.
- [x] 3.3 Implementar actualizacion por retiro incrementando `en_uso` y ajustando `maximo_historico`.
- [x] 3.4 Implementar actualizacion por devolucion decrementando `en_uso`.

## 4. Transacciones

- [x] 4.1 Implementar funcion de persistencia que inserta movimiento y actualiza estado en una misma transaccion.
- [x] 4.2 Asegurar rollback cuando falla cualquiera de las dos escrituras.
- [x] 4.3 Exponer API del adaptador para ser usada por el consumer en un change posterior.

## 5. Tests de integracion

- [x] 5.1 Agregar dependencia de dev `testcontainers`.
- [x] 5.2 Crear helper de PostgreSQL para tests de integracion.
- [x] 5.3 Testear INSERT de recorrido.
- [x] 5.4 Testear UPDATE de estado para retiro y devolucion.
- [x] 5.5 Testear consultas temporales de ultimo y siguiente movimiento.
- [x] 5.6 Testear rollback transaccional.

## 6. Verificacion

- [x] 6.1 Ejecutar `cargo fmt`.
- [x] 6.2 Ejecutar `cargo build`.
- [x] 6.3 Ejecutar `cargo test`.
- [x] 6.4 Ejecutar `sqlx migrate run`.
- [x] 6.5 Validar el change con `openspec validate`.
