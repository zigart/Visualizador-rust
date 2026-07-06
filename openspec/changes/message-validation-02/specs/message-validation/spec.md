## ADDED Requirements

### Requirement: Formato JSON obligatorio
El sistema SHALL rechazar mensajes que no sean JSON válido con objeto raíz.

#### Scenario: JSON inválido
- **WHEN** llega un payload que no es JSON
- **THEN** se loguea error y se NACK el mensaje

### Requirement: Campos obligatorios
El sistema SHALL exigir `id_recorrido`, `id_usuario`, `operacion` y `fechahora` no vacíos.

#### Scenario: Campo faltante
- **WHEN** falta `operacion` en el JSON
- **THEN** se rechaza con `ErrorDominio::CampoObligatorio`

### Requirement: Operación válida
El sistema SHALL aceptar solo operacion `"retiro"` o `"devolucion"`.

#### Scenario: Operación inválida
- **WHEN** operacion es `"X"`
- **THEN** se rechaza con `ErrorDominio::ValorInvalido`

### Requirement: Fecha ISO 8601
El sistema SHALL validar `fechahora` como fecha ISO 8601 parseable.

#### Scenario: Fecha inválida
- **WHEN** `fechahora` es string vacío o no parseable
- **THEN** se rechaza con error de dominio

### Requirement: Coerción de IDs
El sistema SHALL aceptar `id_recorrido` e `id_usuario` como número o string numérico.

#### Scenario: ID como string
- **WHEN** `id_recorrido` es `"880001"`
- **THEN** se interpreta como `880001` antes de validar
