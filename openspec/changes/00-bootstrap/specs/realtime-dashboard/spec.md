## ADDED Requirements

### Requirement: Exponer WebSocket nativo
La API SHALL exponer un endpoint WebSocket nativo con `axum` para clientes del dashboard en tiempo real.

#### Scenario: Cliente conectado
- **WHEN** un cliente inicia una conexion WebSocket valida
- **THEN** la API acepta la conexion y la registra para recibir actualizaciones

### Requirement: Emitir actualizaciones de estado
La API SHALL enviar por WebSocket una actualizacion cada vez que un retiro o devolucion cambia el estado persistido.

#### Scenario: Retiro procesado
- **WHEN** se procesa y persiste un retiro
- **THEN** los clientes WebSocket conectados reciben una actualizacion del cambio

#### Scenario: Devolucion procesada
- **WHEN** se procesa y persiste una devolucion
- **THEN** los clientes WebSocket conectados reciben una actualizacion del cambio

### Requirement: Evitar Socket.IO
La API SHALL usar WebSocket nativo y MUST NOT requerir Socket.IO para conectar clientes.

#### Scenario: Cliente WebSocket estandar
- **WHEN** un cliente usa un protocolo WebSocket estandar
- **THEN** puede conectarse sin handshake ni librerias Socket.IO
