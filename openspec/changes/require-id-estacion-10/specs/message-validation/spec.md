## MODIFIED Requirements

### Requirement: Campos obligatorios
El sistema SHALL exigir `id_recorrido`, `id_usuario`, `id_estacion`, `operacion` y `fechahora` no vacíos; ante incumplimiento ACKea el mensaje y loguea `ErrorDominio::CampoObligatorio`.

#### Scenario: Campo faltante
- **WHEN** falta `operacion` en el JSON
- **THEN** se rechaza con `ErrorDominio::CampoObligatorio`, se loguea y se ACK el mensaje

#### Scenario: id_estacion ausente
- **WHEN** el JSON no incluye la clave `id_estacion`
- **THEN** se rechaza con `ErrorDominio::CampoObligatorio { campo: "id_estacion" }`, se loguea y se ACK el mensaje

#### Scenario: id_estacion null
- **WHEN** `id_estacion` es `null` en el JSON
- **THEN** se rechaza con `ErrorDominio::CampoObligatorio { campo: "id_estacion" }`, se loguea y se ACK el mensaje

#### Scenario: id_estacion vacío
- **WHEN** `id_estacion` es string vacío o solo espacios
- **THEN** se rechaza con `ErrorDominio::CampoObligatorio { campo: "id_estacion" }`, se loguea y se ACK el mensaje

### Requirement: Coerción de IDs
El sistema SHALL aceptar `id_recorrido`, `id_usuario` e `id_estacion` como número o string numérico.

#### Scenario: ID como string
- **WHEN** `id_recorrido` es `"880001"`
- **THEN** se interpreta como `880001` antes de validar

#### Scenario: id_estacion como string
- **WHEN** `id_estacion` es `"15"`
- **THEN** se interpreta como `15` y el movimiento queda listo para persistencia

## ADDED Requirements

### Requirement: id_estacion obligatorio
El sistema SHALL rechazar mensajes cuyo `id_estacion` no sea un entero positivo parseable.

#### Scenario: id_estacion no numérico
- **WHEN** `id_estacion` es `"abc"` u otro valor no numérico
- **THEN** se rechaza con `ErrorDominio::ValorInvalido { campo: "id_estacion", .. }`, se loguea y se ACK el mensaje
