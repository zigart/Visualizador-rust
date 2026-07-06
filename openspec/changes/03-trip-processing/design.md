## Context

El parser de mensajes produce `MovimientoRecorrido` con `id_recorrido`, `id_usuario`, `operacion` y `fechahora`. Antes de persistir, el dominio debe validar reglas de retiros/devoluciones usando historial del usuario y estado agregado.

La persistencia real queda fuera de este change: el dominio dependera de traits y los tests usaran mocks en memoria.

## Goals / Non-Goals

**Goals:**

- Implementar `SistemaBicicletas` como servicio de dominio.
- Implementar `EstadoBicicletas` y el calculo de `bicicletas_disponibles`.
- Definir `RepositorioRecorrido` como puerto de lectura para historial.
- Rechazar retiros y devoluciones invalidas con errores de dominio especificos.
- Cubrir cada escenario con tests unitarios exhaustivos sin I/O.

**Non-Goals:**

- No implementar repositorios PostgreSQL.
- No consumir RabbitMQ ni ACK/NACK en este change.
- No validar estaciones ni usuarios contra catalogos externos.

## Decisions

### Servicio de dominio

`SistemaBicicletas` recibira un `RepositorioRecorrido` y un `EstadoBicicletas`. Procesar un movimiento devolvera un nuevo estado o una decision equivalente, dejando persistencia a adaptadores posteriores.

### Puerto de repositorio

`RepositorioRecorrido` expondra consultas necesarias para reglas:

- buscar por `id_recorrido`
- obtener retiro activo del usuario
- obtener ultimo movimiento antes de una fecha
- obtener siguiente movimiento despues de una fecha

Esto permite validar duplicados y orden temporal sin depender de `sqlx`.

### Estado agregado

`EstadoBicicletas` mantendra `en_uso` y `maximo_historico`. `bicicletas_disponibles` se calculara como `maximo_historico - en_uso`, no se almacenara como valor independiente en dominio.

### Errores de negocio especificos

El dominio agregara errores como `IdRecorridoYaUtilizado`, `RetiroConBicicletaEnUso`, `DevolucionSinRetiroPrevio`, `DevolucionConIdRecorridoDistinto`, `DevolucionConMismaFecha` y errores de orden temporal.

## Risks / Trade-offs

- Las reglas de orden temporal dependen de consultas correctas del repositorio -> los tests mock deben cubrir ultimo y siguiente movimiento.
- Calcular disponibles desde `maximo_historico - en_uso` asume que `maximo_historico` representa el pico observado de bicicletas en uso -> documentar esta semantica en tests.
- Sin persistencia en este change, los repositorios reales pueden revelar ajustes de trait -> mantener la interfaz pequena y orientada a escenarios.
