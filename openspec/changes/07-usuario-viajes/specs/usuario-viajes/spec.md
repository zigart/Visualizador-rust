## ADDED Requirements

### Requirement: Consulta de historial por usuario
El sistema SHALL exponer `GET /usuarios/{id_usuario}` que retorna la colección de viajes del usuario.

#### Scenario: Usuario encontrado
- **GIVEN** un usuario con movimientos registrados en `recorridos`
- **WHEN** se consulta `GET /usuarios/{id_usuario}`
- **THEN** responde 200 OK con body `{ "viajes": [ ... ] }`

#### Scenario: Usuario no encontrado
- **GIVEN** un `id_usuario` sin movimientos en `recorridos`
- **WHEN** se consulta `GET /usuarios/{id_usuario}`
- **THEN** responde 404 Not Found

### Requirement: Orden ascendente por devolución
La colección `viajes` SHALL ordenarse por `fechaHoraDevolucion` ascendente (`NULLS LAST`).

#### Scenario: Colección ordenada
- **GIVEN** un usuario con varios viajes completados en distintas fechas de devolución
- **WHEN** se consulta `GET /usuarios/{id}`
- **THEN** viajes aparecen ordenados por `fechaHoraDevolucion` de menor a mayor

### Requirement: Devolución pendiente con campos null
Si un viaje no tiene devolución, `fechaHoraDevolucion` e `idEstacionDevolucion` SHALL ser null.

#### Scenario: Viaje en curso
- **GIVEN** un retiro sin devolución asociada para `id_recorrido` X
- **WHEN** se consulta el historial del usuario
- **THEN** el viaje X incluye `fechaHoraRetiro` e `idEstacionRetiro` y null en campos de devolución

### Requirement: Estructura de viaje
Cada elemento de `viajes` SHALL incluir: `idRecorrido`, `fechaHoraRetiro`, `idEstacionRetiro`, `fechaHoraDevolucion`, `idEstacionDevolucion`.

#### Scenario: Campos de viaje
- **WHEN** el endpoint retorna un viaje
- **THEN** el objeto contiene los campos requeridos del contrato

## MODIFIED Requirements

### Requirement: Persistencia de id_estacion
El sistema SHALL persistir `id_estacion` en `recorridos` cuando el mensaje RabbitMQ lo incluya.

#### Scenario: Movimiento con estación persistida
- **GIVEN** un mensaje RabbitMQ con `id_estacion`
- **WHEN** el movimiento se persiste en `recorridos`
- **THEN** la fila conserva el `id_estacion` recibido

### Requirement: Mensaje con id_estacion
El adaptador de mensajes SHALL aceptar `id_estacion` opcional/obligatorio según contrato acordado con Procesador.

#### Scenario: Mensaje con estación
- **WHEN** llega un mensaje con `id_estacion`
- **THEN** el adaptador lo parsea y lo deja disponible para persistencia
