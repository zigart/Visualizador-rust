## MODIFIED Requirements

### Requirement: Esquema objetivo de recorridos
El sistema SHALL persistir movimientos en tabla `recorridos` con el siguiente esquema:

```sql
CREATE TABLE recorridos (
    id SERIAL PRIMARY KEY,
    id_recorrido INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    operacion VARCHAR(20) NOT NULL,  -- 'retiro' | 'devolucion'
    fechahora TIMESTAMPTZ NOT NULL,
    id_estacion BIGINT NULL           -- extension Rust; opcional
);
```

La tabla MUST NOT incluir columnas legacy: `bicicleta_id`, `estacion_origen_id`, `estacion_destino_id`, `iniciado_en`, `finalizado_en`, `created_at`, `updated_at`.

#### Scenario: INSERT con columnas del contrato
- **GIVEN** un movimiento valido
- **WHEN** se persiste en `recorridos`
- **THEN** la fila contiene solo `id`, `id_recorrido`, `id_usuario`, `operacion`, `fechahora` e `id_estacion` cuando aplique

#### Scenario: Sin columnas legacy
- **WHEN** se inspecciona el esquema tras migraciones
- **THEN** no existen `bicicleta_id`, `iniciado_en` ni `finalizado_en` en `recorridos`

### Requirement: Esquema objetivo de estado_bicicletas
El sistema SHALL mantener estado agregado en tabla `estado_bicicletas` con el siguiente esquema:

```sql
CREATE TABLE estado_bicicletas (
    id SERIAL PRIMARY KEY,
    en_uso INTEGER NOT NULL DEFAULT 0,
    maximo_historico INTEGER NOT NULL DEFAULT 0
);
```

La tabla MUST NOT incluir `updated_at`, constraint singleton `CHECK (id = 1)` ni checks de no-negativo heredados del bootstrap.

#### Scenario: Estado inicial en cero
- **GIVEN** base migrada sin movimientos
- **WHEN** se lee `estado_bicicletas`
- **THEN** existe fila con `en_uso = 0` y `maximo_historico = 0`

#### Scenario: UPDATE sin updated_at
- **WHEN** se procesa un retiro valido
- **THEN** `en_uso` y `maximo_historico` se actualizan sin escribir `updated_at`

### Requirement: Persistencia de movimientos
El sistema SHALL guardar cada movimiento aceptado en tabla `recorridos` usando unicamente columnas del contrato objetivo.

#### Scenario: INSERT de recorrido
- **GIVEN** retiro valido procesado
- **WHEN** se persiste
- **THEN** existe fila en `recorridos` con `id_recorrido`, `id_usuario`, `operacion`, `fechahora`

### Requirement: Persistencia de estado agregado
El sistema SHALL actualizar `estado_bicicletas` tras cada movimiento aceptado.

#### Scenario: UPDATE de estado
- **GIVEN** `en_uso=3`, `maximo_historico=5`
- **WHEN** se procesa devolucion valida
- **THEN** `en_uso=2` en la base de datos

### Requirement: Consultas temporales por usuario
El sistema SHALL consultar ultimo y siguiente movimiento por usuario y fecha para validar orden.

#### Scenario: Buscar ultimo movimiento
- **GIVEN** movimientos del usuario 1 en distintas fechas
- **WHEN** se busca ultimo antes de fecha X
- **THEN** retorna el movimiento inmediatamente anterior a X

### Requirement: Transaccionalidad
El sistema SHALL persistir movimiento y estado en la misma transaccion.

#### Scenario: Rollback ante error
- **GIVEN** un movimiento valido y un error al actualizar estado
- **WHEN** falla la transaccion
- **THEN** no queda insertado el movimiento en `recorridos`
