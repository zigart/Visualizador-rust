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
    id_estacion BIGINT NOT NULL
);
```

La tabla MUST NOT incluir columnas legacy: `bicicleta_id`, `estacion_origen_id`, `estacion_destino_id`, `iniciado_en`, `finalizado_en`, `created_at`, `updated_at`.

#### Scenario: INSERT con columnas del contrato
- **GIVEN** un movimiento valido con `id_estacion`
- **WHEN** se persiste en `recorridos`
- **THEN** la fila contiene `id`, `id_recorrido`, `id_usuario`, `operacion`, `fechahora` e `id_estacion` no nulo

#### Scenario: Sin columnas legacy
- **WHEN** se inspecciona el esquema tras migraciones
- **THEN** no existen `bicicleta_id`, `iniciado_en` ni `finalizado_en` en `recorridos`

### Requirement: Persistencia de movimientos
El sistema SHALL guardar cada movimiento aceptado en tabla `recorridos` usando unicamente columnas del contrato objetivo, incluyendo `id_estacion` obligatorio.

#### Scenario: INSERT de recorrido
- **GIVEN** retiro valido procesado con `id_estacion = 10`
- **WHEN** se persiste
- **THEN** existe fila en `recorridos` con `id_recorrido`, `id_usuario`, `operacion`, `fechahora` e `id_estacion = 10`
