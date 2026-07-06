## Why

Visualizador ya puede parsear movimientos entrantes, pero todavia necesita reglas de negocio para aceptar o rechazar retiros y devoluciones antes de persistirlos. Este change define el sistema de bicicletas en dominio, el estado agregado y los puertos necesarios para consultar movimientos previos.

## What Changes

- Agregar `SistemaBicicletas` como servicio de dominio para procesar retiros y devoluciones.
- Agregar `EstadoBicicletas` con `en_uso`, `maximo_historico` y calculo de disponibles.
- Agregar errores de negocio especificos para duplicados, retiros activos y devoluciones invalidas.
- Definir trait `RepositorioRecorrido` para consultar movimientos existentes sin acoplar dominio a PostgreSQL.
- Agregar tests unitarios exhaustivos con repositorio mock.

## Capabilities

### New Capabilities

- `trip-processing`: Reglas de negocio para retiros, devoluciones, orden temporal y estado agregado de bicicletas.

### Modified Capabilities

- None.

## Impact

- Actualiza `crates/dominio` con reglas y puertos.
- Agrega tests unitarios de dominio sin I/O.
- Prepara el contrato que implementaran los repositorios PostgreSQL en un change posterior.
