## Context

El consumer RabbitMQ actual usa `confirmation_for_result`: exito → ACK, cualquier error → NACK con requeue. Eso aplica tanto a JSON invalido como a `ErrorDominio`, generando reintentos sobre mensajes que nunca seran validos.

El proyecto ya usa `tracing` con `LOG_FORMAT=json` para agregadores. Sumo Logic suele ingerir logs via HTTP Source o collector sobre stdout JSON en Kubernetes.

## Goals / Non-Goals

**Goals:**

- ACKear mensajes con error de validacion (parseo + dominio).
- NACKear solo fallos transitorios (PostgreSQL, RabbitMQ u otros errores de infraestructura).
- Loguear errores de validacion con campos estructurados.
- Enviar logs a Sumo Logic cuando `SUMOLOGIC_ENDPOINT` este configurado.
- Fallback automatico a consola si Sumo Logic no esta disponible.

**Non-Goals:**

- No implementar DLQ dedicada.
- No cambiar el contrato de mensajes JSON.
- No reintentar envio a Sumo Logic de forma asincrona con cola propia (primer incremento: fire-and-forget con fallback).

## Decisions

### Clasificacion de errores

Introducir `ErrorProcesamiento` en `api` o `adaptadores` con variantes:

- `Validacion` — parseo (`ErrorDominio` de mensajes) o dominio (`ErrorPersistencia::Dominio`). Siempre ACK.
- `Transitorio` — DB, broker, dashboard broadcast. NACK con requeue.

El handler del consumer retornara `Ok(())` ante validacion fallida (tras loguear) en lugar de `Err`, o el adaptador RabbitMQ recibira una senal explicita de confirmacion.

**Alternativa considerada**: ACK universal para todo error. Descartada porque errores de DB deben reintentarse.

### Politica ACK en `rabbitmq.rs`

Reemplazar `confirmation_for_result` por `confirmation_for_processing(ProcessingOutcome)`:

```rust
enum ProcessingOutcome {
    Success,
    ValidationError,
    TransientError,
}
```

- `Success` | `ValidationError` → ACK
- `TransientError` → NACK requeue

### Logging a Sumo Logic

Agregar capa `SumoLogicLayer` en `tracing-subscriber` como `Layer` opcional:

- Variables: `SUMOLOGIC_ENDPOINT` (URL HTTP), `SUMOLOGIC_SOURCE_NAME` (opcional).
- En cada evento `ERROR` de validacion, el layer intenta POST JSON al endpoint.
- Si falla o no hay endpoint, el layer `fmt`/`json` existente sigue escribiendo a consola.

**Alternativa considerada**: solo stdout JSON y collector externo de Sumo. Se complementa: el collector cubre logs generales; el HTTP layer cubre el requisito explicito de envio a Sumo Logic desde la app.

**Alternativa considerada**: OpenTelemetry exporter. Descartada por simplicidad y porque el stack ya usa `tracing-subscriber`.

### Formato del evento Sumo Logic

Payload JSON con campos:

```json
{
  "service": "visualizador-rust",
  "level": "error",
  "event": "validacion_movimiento",
  "origen": "parseo|dominio",
  "id_recorrido": "...",
  "id_usuario": "...",
  "operacion": "...",
  "error": "...",
  "timestamp": "ISO8601"
}
```

## Risks / Trade-offs

- [Mensajes invalidos se pierden de la cola sin DLQ] → Mitigacion: logs estructurados en Sumo Logic + consola con payload resumido.
- [Latencia extra por POST a Sumo Logic] → Mitigacion: envio async fire-and-forget; no bloquear ACK.
- [Duplicacion de logs en consola y Sumo Logic] → Aceptable en primer incremento; el requisito pide fallback a consola.

## Migration Plan

1. Desplegar con `SUMOLOGIC_ENDPOINT` vacio: solo cambia politica ACK, logs en consola.
2. Configurar endpoint Sumo Logic en staging y verificar ingesta.
3. Desplegar a produccion; monitorear tasa de ACK por validacion vs NACK transitorio.

Rollback: revertir a `confirmation_for_result` anterior si hay regresion (mensajes validos ACKeados por error).

## Open Questions

- ¿Incluir hash o truncado del payload completo en el log para auditoria forense?
- ¿Timeout recomendado para POST a Sumo Logic (ej. 2s)?
