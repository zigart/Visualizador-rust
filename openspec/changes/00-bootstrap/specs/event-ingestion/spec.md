## ADDED Requirements

### Requirement: Consumir eventos de bicicletas desde RabbitMQ
El sistema SHALL consumir mensajes JSON desde la cola RabbitMQ `bike_trips` representando retiros y devoluciones de bicicletas.

#### Scenario: Mensaje de retiro recibido
- **WHEN** RabbitMQ entrega un mensaje valido de tipo retiro en `bike_trips`
- **THEN** el sistema procesa el retiro mediante el puerto de aplicacion correspondiente

#### Scenario: Mensaje de devolucion recibido
- **WHEN** RabbitMQ entrega un mensaje valido de tipo devolucion en `bike_trips`
- **THEN** el sistema procesa la devolucion mediante el puerto de aplicacion correspondiente

### Requirement: Confirmar mensajes luego de procesarlos
El sistema SHALL confirmar un mensaje RabbitMQ solo despues de aplicar reglas de negocio y persistir el cambio requerido.

#### Scenario: Procesamiento exitoso
- **WHEN** un mensaje valido se procesa y persiste correctamente
- **THEN** el sistema confirma el mensaje ante RabbitMQ

#### Scenario: Procesamiento fallido
- **WHEN** un mensaje no puede procesarse o persistirse
- **THEN** el sistema no confirma el mensaje como exitoso

### Requirement: Aislar el transporte del dominio
El consumidor RabbitMQ SHALL transformar mensajes externos en comandos o eventos de dominio sin exponer dependencias de RabbitMQ al crate `dominio`.

#### Scenario: Dominio sin dependencia de RabbitMQ
- **WHEN** se compila el crate `dominio`
- **THEN** no requiere tipos ni dependencias de `lapin`
