## ADDED Requirements

### Requirement: Suscripción a cola al arrancar
El servicio SHALL conectarse a RabbitMQ al iniciar y consumir de la cola configurada (`QUEUE_NAME`, default `bike_trips`).

#### Scenario: Consumer activo
- **WHEN** el servicio arranca con RabbitMQ disponible
- **THEN** se suscribe a la cola y procesa mensajes entrantes

### Requirement: Confirmación manual de mensajes
El servicio SHALL confirmar (ACK) mensajes solo tras procesamiento exitoso.

#### Scenario: ACK tras éxito
- **GIVEN** un mensaje válido procesado sin error
- **WHEN** finaliza el procesamiento
- **THEN** se envía ACK al broker

### Requirement: Reencolado ante error
El servicio SHALL reencolar (NACK requeue=true) mensajes que fallen al procesarse.

#### Scenario: NACK en error
- **GIVEN** un mensaje que produce error de parseo o dominio
- **WHEN** falla el procesamiento
- **THEN** se envía NACK con requeue

### Requirement: Reconexión automática
El servicio SHALL reconectarse automáticamente ante caídas del broker.

#### Scenario: Recuperación tras caída
- **GIVEN** el servicio consumiendo mensajes
- **WHEN** RabbitMQ se reinicia
- **THEN** el consumer se reconecta y reanuda consumo

### Requirement: Prefetch configurable
El servicio SHALL respetar `RABBIT_PREFETCH` (default `1`) vía `basic_qos`.

#### Scenario: Prefetch aplicado
- **WHEN** el servicio crea el canal RabbitMQ
- **THEN** configura `basic_qos` con el valor de `RABBIT_PREFETCH`
