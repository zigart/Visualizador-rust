## ADDED Requirements

### Requirement: Registro de retiro válido
El sistema SHALL incrementar `en_uso` en 1 al procesar un retiro válido.

#### Scenario: Retiro exitoso
- **GIVEN** usuario sin bicicleta en uso y `id_recorrido` no utilizado
- **WHEN** llega operacion retiro
- **THEN** `en_uso` aumenta en 1 y se actualiza `maximo_historico` si corresponde

### Requirement: Rechazo de id_recorrido duplicado
El sistema SHALL rechazar retiros con `id_recorrido` ya existente.

#### Scenario: ID duplicado
- **GIVEN** `id_recorrido` 1 ya registrado
- **WHEN** llega otro retiro con `id_recorrido` 1
- **THEN** `ErrorDominio::IdRecorridoYaUtilizado`

### Requirement: Rechazo de retiro con bicicleta en uso
El sistema SHALL rechazar retiro si el usuario ya tiene retiro activo sin devolución.

#### Scenario: Segundo retiro sin devolución
- **GIVEN** usuario con retiro previo sin devolución
- **WHEN** llega nuevo retiro del mismo usuario
- **THEN** `ErrorDominio::RetiroConBicicletaEnUso`

### Requirement: Rechazo de retiro fuera de orden temporal
El sistema SHALL rechazar retiro si existe retiro posterior del mismo usuario.

#### Scenario: Retiro posterior existente
- **GIVEN** usuario con retiro posterior a la fecha del mensaje entrante
- **WHEN** llega un retiro con fecha anterior
- **THEN** se rechaza con error de dominio

### Requirement: Registro de devolución válida
El sistema SHALL decrementar `en_uso` en 1 al procesar devolución válida.

#### Scenario: Devolución exitosa
- **GIVEN** retiro previo con mismo `id_recorrido` y fecha distinta
- **WHEN** llega operacion devolucion
- **THEN** `en_uso` disminuye en 1

### Requirement: Devolución sin retiro previo
El sistema SHALL rechazar devolución sin retiro previo del usuario.

#### Scenario: Sin retiro previo
- **GIVEN** usuario sin retiro previo
- **WHEN** llega operacion devolucion
- **THEN** se rechaza con error de dominio

### Requirement: Devolución con id_recorrido distinto
El sistema SHALL rechazar devolución cuyo `id_recorrido` no coincide con el retiro asociado.

#### Scenario: ID no coincide
- **GIVEN** usuario con retiro activo de `id_recorrido` 1
- **WHEN** llega devolucion con `id_recorrido` 2
- **THEN** se rechaza con error de dominio

### Requirement: Devolución con misma fecha que retiro
El sistema SHALL rechazar devolución con `fechahora` igual al retiro.

#### Scenario: Misma fecha
- **GIVEN** retiro previo en fecha X
- **WHEN** llega devolucion del mismo recorrido en fecha X
- **THEN** se rechaza con error de dominio

### Requirement: Devolución duplicada en el tiempo
El sistema SHALL rechazar devolución si ya existe devolución posterior del usuario.

#### Scenario: Devolución posterior existente
- **GIVEN** usuario con devolución posterior a la fecha del mensaje entrante
- **WHEN** llega una devolución con fecha anterior
- **THEN** se rechaza con error de dominio

### Requirement: Cálculo de bicicletas disponibles
El sistema SHALL calcular `bicicletas_disponibles` como `maximo_historico - en_uso`.

#### Scenario: Disponibles calculadas
- **GIVEN** `maximo_historico=5` y `en_uso=2`
- **WHEN** se consulta `bicicletas_disponibles`
- **THEN** retorna `3`
