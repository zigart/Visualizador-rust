## ADDED Requirements

### Requirement: Servir dashboard estático
El sistema SHALL servir HTML/CSS/JS en `GET /dashboard`.

#### Scenario: Página accesible
- **WHEN** se accede a `/dashboard`
- **THEN** retorna `index.html` con indicadores y historial de eventos

### Requirement: WebSocket de estado
El sistema SHALL exponer endpoint WebSocket `/ws` que envía JSON `EstadoDashboard`.

#### Scenario: Estado al conectar
- **GIVEN** estado persistido en DB
- **WHEN** un cliente abre WebSocket
- **THEN** recibe inmediatamente el estado actual

### Requirement: Broadcast tras movimiento
El sistema SHALL emitir `EstadoDashboard` a todos los clientes tras cada movimiento válido.

#### Scenario: Actualización en vivo
- **GIVEN** cliente conectado al WebSocket
- **WHEN** se procesa un retiro válido
- **THEN** el cliente recibe JSON con `bicicletas_disponibles`, `bicicletas_en_uso`, `actualizado_en`, `movimiento`

### Requirement: Payload EstadoDashboard
El payload SHALL incluir: `bicicletas_disponibles`, `bicicletas_en_uso`, `actualizado_en` (ISO), `estado_conexion`, `movimiento` opcional.

#### Scenario: Payload completo
- **WHEN** el sistema serializa `EstadoDashboard`
- **THEN** el JSON contiene indicadores, timestamp ISO, estado de conexión y campo `movimiento`

### Requirement: Indicadores en UI
El dashboard SHALL mostrar bicicletas disponibles, en uso, última actualización (formato es-AR) y estado de conexión.

#### Scenario: Indicadores renderizados
- **WHEN** la UI recibe `EstadoDashboard`
- **THEN** actualiza disponibles, en uso, última actualización y estado de conexión

### Requirement: Historial de eventos
El dashboard SHALL agregar cada movimiento recibido a una lista visible sin recargar la página.

#### Scenario: Evento agregado
- **WHEN** llega un `EstadoDashboard` con `movimiento`
- **THEN** la UI agrega el movimiento al historial visible sin recargar
