## Context

Los cambios anteriores dejaron el workspace, la conexion a RabbitMQ y el parseo de mensajes JSON hacia movimientos validados. Falta persistir cada movimiento aceptado y mantener un estado agregado en PostgreSQL.

La migracion bootstrap actual tiene una forma inicial de `recorridos` y `estado_bicicletas`; este change debe alinear el esquema con el contrato de mensajes validado: `id_recorrido`, `id_usuario`, `operacion` y `fechahora`.

## Goals / Non-Goals

**Goals:**

- Insertar movimientos aceptados en `recorridos`.
- Actualizar `estado_bicicletas.id = 1` tras cada movimiento.
- Consultar ultimo y siguiente movimiento por usuario y fecha.
- Garantizar que movimiento y estado se escriben en una unica transaccion.
- Cubrir repositorios con tests de integracion usando `testcontainers`.

**Non-Goals:**

- No implementar broadcast WebSocket.
- No implementar validacion de estaciones.
- No implementar idempotencia avanzada ni DLQ.

## Decisions

### Repositorios en adaptadores

Los repositorios PostgreSQL viviran en `crates/adaptadores`, porque dependen de `sqlx::PgPool` y SQL concreto. Expondran metodos orientados al dominio para insertar movimientos, consultar vecinos temporales y actualizar estado.

### Transacciones explicitas

El caso de uso de persistencia recibira una transaccion `sqlx` o abrira una transaccion desde el pool. Dentro de la misma transaccion se insertara el movimiento y se actualizara `estado_bicicletas`.

### Estado singleton

`estado_bicicletas` usara la fila singleton `id = 1`. `retiro` incrementara `en_uso`; `devolucion` decrementara `en_uso`; `maximo_historico` se mantendra como el maximo observado de `en_uso`.

### Consultas temporales

Las consultas por usuario ordenaran por `fechahora` e identificador estable para resolver empates. `ultimo_antes_de` retornara el movimiento inmediatamente anterior a una fecha dada; `siguiente_despues_de` retornara el inmediatamente posterior.

## Risks / Trade-offs

- El esquema bootstrap no coincide plenamente con el contrato de movimiento -> agregar una migracion incremental o ajustar la migracion inicial si aun no esta archivada.
- Tests con `testcontainers` requieren Docker activo -> documentar prerequisitos y mantener tests unitarios separados cuando sea posible.
- La actualizacion de estado agregado puede quedar incorrecta si se aplican movimientos fuera de orden -> usar consultas temporales para validar orden antes de persistir en cambios posteriores.
