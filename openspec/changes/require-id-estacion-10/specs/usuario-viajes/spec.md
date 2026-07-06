## MODIFIED Requirements

### Requirement: Persistencia de id_estacion
El sistema SHALL persistir `id_estacion` en `recorridos` para todo mensaje RabbitMQ aceptado; el valor MUST ser el recibido en el mensaje.

#### Scenario: Movimiento con estación persistida
- **GIVEN** un mensaje RabbitMQ con `id_estacion = 42`
- **WHEN** el movimiento se persiste en `recorridos`
- **THEN** la fila conserva `id_estacion = 42`

### Requirement: Mensaje con id_estacion
El adaptador de mensajes SHALL exigir `id_estacion` presente y valido en todo mensaje entrante segun contrato con Procesador.

#### Scenario: Mensaje con estación obligatoria
- **WHEN** llega un mensaje con `id_estacion` valido
- **THEN** el adaptador lo parsea como `u64` y lo deja disponible para persistencia

#### Scenario: Mensaje sin estación rechazado
- **WHEN** llega un mensaje sin `id_estacion`, con `null` o string vacío
- **THEN** el adaptador rechaza el mensaje antes de persistir
