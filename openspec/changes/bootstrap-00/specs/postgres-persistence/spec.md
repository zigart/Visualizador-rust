## ADDED Requirements

### Requirement: Crear migraciones iniciales
El sistema SHALL incluir migraciones SQL iniciales para las tablas `recorridos` y `estado_bicicletas`.

#### Scenario: Migraciones aplicadas
- **WHEN** se ejecutan las migraciones sobre una base PostgreSQL vacia
- **THEN** existen las tablas `recorridos` y `estado_bicicletas`

### Requirement: Persistir recorridos
El adaptador PostgreSQL SHALL persistir el inicio y cierre de recorridos con identificador de bicicleta, estaciones y timestamps.

#### Scenario: Inicio persistido
- **WHEN** se registra un retiro valido
- **THEN** existe un recorrido abierto asociado a la bicicleta

#### Scenario: Cierre persistido
- **WHEN** se registra una devolucion valida
- **THEN** el recorrido activo queda cerrado con datos de finalizacion

### Requirement: Mantener estado actual de bicicletas
El adaptador PostgreSQL SHALL mantener en `estado_bicicletas` el ultimo estado conocido de cada bicicleta.

#### Scenario: Estado actualizado por retiro
- **WHEN** se persiste un retiro valido
- **THEN** el estado actual de la bicicleta indica que esta retirada

#### Scenario: Estado actualizado por devolucion
- **WHEN** se persiste una devolucion valida
- **THEN** el estado actual de la bicicleta indica que esta disponible
