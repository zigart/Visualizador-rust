## ADDED Requirements

### Requirement: Registrar retiro de bicicleta
El dominio SHALL registrar un retiro abriendo un recorrido activo para la bicicleta y marcando su estado como retirada.

#### Scenario: Retiro valido
- **WHEN** se procesa un retiro para una bicicleta sin recorrido activo
- **THEN** queda registrado un recorrido activo con estacion y momento de inicio

#### Scenario: Retiro duplicado
- **WHEN** se procesa un retiro para una bicicleta que ya tiene un recorrido activo
- **THEN** el dominio rechaza la operacion con un error de negocio

### Requirement: Registrar devolucion de bicicleta
El dominio SHALL registrar una devolucion cerrando el recorrido activo de la bicicleta y marcando su estado como disponible.

#### Scenario: Devolucion valida
- **WHEN** se procesa una devolucion para una bicicleta con recorrido activo
- **THEN** el recorrido queda cerrado con estacion y momento de fin

#### Scenario: Devolucion sin recorrido activo
- **WHEN** se procesa una devolucion para una bicicleta sin recorrido activo
- **THEN** el dominio rechaza la operacion con un error de negocio

### Requirement: Definir puertos de dominio
El crate `dominio` SHALL definir traits para persistir recorridos, consultar estado de bicicletas y publicar actualizaciones sin depender de implementaciones concretas.

#### Scenario: Adaptador implementa puerto
- **WHEN** un adaptador necesita persistir estado
- **THEN** implementa los traits definidos por `dominio`
