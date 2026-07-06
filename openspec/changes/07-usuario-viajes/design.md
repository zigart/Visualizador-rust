## Context

Los movimientos procesados contienen `id_recorrido`, `id_usuario`, `operacion` y `fechahora`. Para consultar viajes por usuario se debe reconstruir cada viaje agrupando retiro y devolucion por `id_recorrido`, incluyendo estacion de retiro/devolucion cuando exista.

## Goals / Non-Goals

**Goals:**

- Exponer `GET /usuarios/{id_usuario}`.
- Retornar `404` cuando el usuario no tiene movimientos.
- Ordenar viajes por `fechaHoraDevolucion ASC NULLS LAST`.
- Representar viajes en curso con campos de devolucion `null`.
- Agregar soporte de `id_estacion` en persistencia y mensaje.

**Non-Goals:**

- No implementar paginacion en este change.
- No validar estaciones contra un catalogo externo.
- No agregar autenticacion al endpoint de consulta.

## Decisions

### Servicio de armado de viajes

El servicio de consulta recibira movimientos ordenables desde `ConsultaRepositorioRecorrido::listar_viajes_por_usuario` o una consulta equivalente. La respuesta HTTP usara camelCase: `idRecorrido`, `fechaHoraRetiro`, `idEstacionRetiro`, `fechaHoraDevolucion`, `idEstacionDevolucion`.

### Agrupacion por id_recorrido

Cada viaje se arma agrupando retiro y devolucion por `id_recorrido`. Si solo existe retiro, los campos de devolucion seran `null`. Si existe devolucion asociada, se completan fecha y estacion de devolucion.

### Orden

El orden final sera `fechaHoraDevolucion ASC NULLS LAST`. Los viajes en curso quedan al final; ante empates se puede ordenar por `idRecorrido` para estabilidad.

### id_estacion

`id_estacion` debe agregarse al contrato de mensaje y persistirse en `recorridos` para poder informar estaciones de retiro/devolucion. Hasta que se confirme si es obligatorio u opcional, la spec permite ajustar validacion segun contrato con Procesador.

## Risks / Trade-offs

- Si `id_estacion` entra como opcional, algunos viajes historicos podrian tener estacion `null` aunque la respuesta espera campo presente -> representar como nullable o migrar datos cuando exista fuente.
- Sin paginacion, usuarios con muchos viajes pueden generar respuestas grandes -> aceptable para primer corte.
- Agrupar movimientos inconsistentes puede exponer datos parciales -> confiar en reglas de negocio previas y agregar tests de casos esperados.
