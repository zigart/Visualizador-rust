## 1. Esquema PostgreSQL

- [ ] 1.1 Revisar la migracion actual y definir cambios necesarios para `id_recorrido`, `id_usuario`, `operacion`, `fechahora`, `en_uso` y `maximo_historico`.
- [ ] 1.2 Crear migracion incremental o ajustar bootstrap si corresponde al flujo del repo.
- [ ] 1.3 Verificar que `sqlx migrate run` deja `recorridos` y `estado_bicicletas` con columnas requeridas.

## 2. RepositorioRecorrido

- [ ] 2.1 Definir tipo de fila o modelo persistido para movimientos.
- [ ] 2.2 Implementar INSERT de movimiento aceptado en `recorridos`.
- [ ] 2.3 Implementar consulta de ultimo movimiento por usuario antes de una fecha.
- [ ] 2.4 Implementar consulta de siguiente movimiento por usuario despues de una fecha.

## 3. RepositorioEstadoBicicletas

- [ ] 3.1 Definir modelo de estado agregado con `en_uso` y `maximo_historico`.
- [ ] 3.2 Implementar lectura de la fila singleton `id=1`.
- [ ] 3.3 Implementar actualizacion por retiro incrementando `en_uso` y ajustando `maximo_historico`.
- [ ] 3.4 Implementar actualizacion por devolucion decrementando `en_uso`.

## 4. Transacciones

- [ ] 4.1 Implementar funcion de persistencia que inserta movimiento y actualiza estado en una misma transaccion.
- [ ] 4.2 Asegurar rollback cuando falla cualquiera de las dos escrituras.
- [ ] 4.3 Exponer API del adaptador para ser usada por el consumer en un change posterior.

## 5. Tests de integracion

- [ ] 5.1 Agregar dependencia de dev `testcontainers`.
- [ ] 5.2 Crear helper de PostgreSQL para tests de integracion.
- [ ] 5.3 Testear INSERT de recorrido.
- [ ] 5.4 Testear UPDATE de estado para retiro y devolucion.
- [ ] 5.5 Testear consultas temporales de ultimo y siguiente movimiento.
- [ ] 5.6 Testear rollback transaccional.

## 6. Verificacion

- [ ] 6.1 Ejecutar `cargo fmt`.
- [ ] 6.2 Ejecutar `cargo build`.
- [ ] 6.3 Ejecutar `cargo test`.
- [ ] 6.4 Ejecutar `sqlx migrate run`.
- [ ] 6.5 Validar el change con `openspec validate`.
