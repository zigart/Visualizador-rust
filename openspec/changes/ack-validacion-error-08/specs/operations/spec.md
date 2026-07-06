## MODIFIED Requirements

### Requirement: Logging estructurado
El sistema SHALL loguear inicio/fin de procesamiento y errores de validacion (parseo y dominio) con `tracing`, enviando a Sumo Logic cuando corresponda y a consola como fallback.

#### Scenario: Error de dominio logueado
- **WHEN** falla validacion de negocio
- **THEN** `tracing::error` incluye `id_recorrido`, `id_usuario`, `operacion`, `origen` y mensaje de error

#### Scenario: Error de parseo logueado
- **WHEN** falla el parseo del payload
- **THEN** `tracing::error` incluye `origen=parseo` y el detalle del error

## ADDED Requirements

### Requirement: Clasificacion de errores para ACK
El sistema SHALL distinguir errores de validacion (ACK) de errores transitorios de infraestructura (NACK opcional).

#### Scenario: Error transitorio de base de datos
- **WHEN** falla la persistencia por error de conexion PostgreSQL
- **THEN** se envia NACK con requeue y se loguea como error de infraestructura

#### Scenario: Error de validacion de negocio
- **WHEN** `SistemaBicicletas` rechaza el movimiento
- **THEN** se envia ACK y se loguea como error de validacion
