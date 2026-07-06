## Context

El contrato esperado de RabbitMQ es:

```json
{"id_recorrido":1,"id_usuario":1,"operacion":"retiro","fechahora":"2026-06-08T15:34:20Z"}
```

Antes de que el mensaje llegue a reglas de negocio o persistencia, el sistema debe convertir el payload crudo en un comando validado. Los errores deben ser tipados para que el consumer pueda loguear y aplicar NACK segun el resultado.

## Goals / Non-Goals

**Goals:**

- Rechazar payloads que no sean JSON valido con objeto raiz.
- Exigir campos no vacios.
- Aceptar solo `retiro` y `devolucion`.
- Parsear `fechahora` como ISO 8601.
- Coercionar IDs numericos recibidos como numero o string numerico.
- Reportar errores como `ErrorDominio::CampoObligatorio` o `ErrorDominio::ValorInvalido`.

**Non-Goals:**

- No validar existencia de usuario, estacion ni recorrido contra base de datos.
- No definir politica de DLQ; el consumer decide NACK/requeue segun cambios posteriores.
- No persistir eventos en este change.

## Decisions

### Parser aislado del transporte

El parser vivira fuera de tipos concretos de `lapin`. Recibira bytes o string y devolvera un mensaje validado o `ErrorDominio`, permitiendo tests unitarios sin RabbitMQ.

### DTO flexible para IDs

`id_recorrido` e `id_usuario` aceptaran JSON number o string numerico. La coercion se hara antes de validar valores para devolver errores consistentes.

### Operacion tipada

`operacion` se convertira a enum de dominio o tipo equivalente con variantes `Retiro` y `Devolucion`. Cualquier otro valor devolvera `ErrorDominio::ValorInvalido`.

### Fecha ISO 8601

`fechahora` se parseara con tipo temporal compatible con ISO 8601, por ejemplo `chrono::DateTime<Utc>`.

## Risks / Trade-offs

- Coercionar strings numericos puede ocultar inconsistencias del productor -> se acepta porque la spec requiere compatibilidad con ambos formatos.
- Reencolar errores de parseo puede repetir mensajes invalidos indefinidamente -> se mantiene porque el change de consumer define NACK con requeue; una mejora futura puede agregar DLQ.
