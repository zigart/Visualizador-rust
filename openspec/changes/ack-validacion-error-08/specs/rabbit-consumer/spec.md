## MODIFIED Requirements

### Requirement: Confirmación manual de mensajes
El servicio SHALL confirmar (ACK) mensajes tras procesamiento exitoso o ante errores de validacion; los fallos transitorios de infraestructura pueden usar NACK con requeue.

#### Scenario: ACK tras éxito
- **GIVEN** un mensaje válido procesado y persistido sin error
- **WHEN** finaliza el procesamiento
- **THEN** se envía ACK al broker

#### Scenario: ACK en error de validacion
- **GIVEN** un mensaje con error de parseo o de reglas de negocio
- **WHEN** falla la validacion
- **THEN** se envía ACK al broker y se loguea el error

## REMOVED Requirements

### Requirement: Reencolado ante error
**Reason**: Los errores de validacion son definitivos; reencolar genera loops infinitos.
**Migration**: Los mensajes invalidos se ACKean y se registran en logs; solo errores transitorios (DB/broker) usan NACK.
