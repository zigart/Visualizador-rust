## ADDED Requirements

### Requirement: Persistencia de movimientos
El sistema SHALL guardar cada movimiento aceptado en tabla `recorridos`.

#### Scenario: INSERT de recorrido
- **GIVEN** retiro válido procesado
- **WHEN** se persiste
- **THEN** existe fila en `recorridos` con `id_recorrido`, `id_usuario`, `operacion`, `fechahora`

### Requirement: Persistencia de estado agregado
El sistema SHALL actualizar `estado_bicicletas` (fila `id=1`) tras cada movimiento.

#### Scenario: UPDATE de estado
- **GIVEN** `en_uso=3`, `maximo_historico=5`
- **WHEN** se procesa devolución válida
- **THEN** `en_uso=2` en la base de datos

### Requirement: Consultas temporales por usuario
El sistema SHALL consultar último y siguiente movimiento por usuario y fecha para validar orden.

#### Scenario: Buscar último movimiento
- **GIVEN** movimientos del usuario 1 en distintas fechas
- **WHEN** se busca último antes de fecha X
- **THEN** retorna el movimiento inmediatamente anterior a X

### Requirement: Transaccionalidad
El sistema SHALL persistir movimiento y estado en la misma transacción.

#### Scenario: Rollback ante error
- **GIVEN** un movimiento válido y un error al actualizar estado
- **WHEN** falla la transacción
- **THEN** no queda insertado el movimiento en `recorridos`
