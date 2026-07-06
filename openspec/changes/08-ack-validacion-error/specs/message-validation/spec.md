## MODIFIED Requirements

### Requirement: Formato JSON obligatorio
El sistema SHALL rechazar mensajes que no sean JSON válido con objeto raíz y confirmar (ACK) el mensaje tras loguear el error.

#### Scenario: JSON inválido
- **WHEN** llega un payload que no es JSON
- **THEN** se loguea error y se ACK el mensaje

### Requirement: Campos obligatorios
El sistema SHALL exigir `id_recorrido`, `id_usuario`, `operacion` y `fechahora` no vacíos; ante incumplimiento ACKea el mensaje y loguea `ErrorDominio::CampoObligatorio`.

#### Scenario: Campo faltante
- **WHEN** falta `operacion` en el JSON
- **THEN** se rechaza con `ErrorDominio::CampoObligatorio`, se loguea y se ACK el mensaje

### Requirement: Operación válida
El sistema SHALL aceptar solo operacion `"retiro"` o `"devolucion"`; valores invalidos generan log y ACK.

#### Scenario: Operación inválida
- **WHEN** operacion es `"X"`
- **THEN** se rechaza con `ErrorDominio::ValorInvalido`, se loguea y se ACK el mensaje

### Requirement: Fecha ISO 8601
El sistema SHALL validar `fechahora` como fecha ISO 8601 parseable; fechas invalidas generan log y ACK.

#### Scenario: Fecha inválida
- **WHEN** `fechahora` es string vacío o no parseable
- **THEN** se rechaza con error de dominio, se loguea y se ACK el mensaje
