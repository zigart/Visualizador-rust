## Context

En `usuario-viajes-07` se agregó `id_estacion` como campo opcional en mensaje y columna nullable en PostgreSQL, pendiente de cerrar contrato con Procesador. El parser usa `parse_optional_id`, que acepta ausencia, `null` y string vacío. El dominio modela `id_estacion: Option<u64>`.

Con el contrato cerrado, Procesador siempre envía `id_estacion`; mensajes sin él son inválidos y deben ACKearse como error de validación (política de `ack-validacion-error-08`).

## Goals / Non-Goals

**Goals:**

- Tratar `id_estacion` igual que `id_recorrido` / `id_usuario`: obligatorio, no vacío, coercible desde número o string numérico.
- Cambiar `MovimientoRecorrido.id_estacion` a `u64`.
- Columna `recorridos.id_estacion` NOT NULL en base de datos.
- Tests unitarios e integración actualizados.

**Non-Goals:**

- Validar que la estación exista en un catálogo externo.
- Cambiar la semántica de `idEstacionDevolucion` null en viajes en curso (solo aplica a la respuesta HTTP, no al mensaje entrante).
- Reprocesar histórico de mensajes ya ACKeados sin estación.

## Decisions

### 1. Reutilizar `parse_required_id` para `id_estacion`

**Decisión:** Reemplazar `parse_optional_id` por `parse_required_id` en `validacion_mensajes.rs`.

**Alternativa descartada:** Mantener `Option` en dominio y fallar solo si es `None` — añade ramas innecesarias en persistencia y sistema.

### 2. Tipo de dominio `u64` no opcional

**Decisión:** `MovimientoRecorrido.id_estacion: u64`.

**Rationale:** Alineado con otros IDs obligatorios; simplifica INSERT sin `.map()`.

### 3. Migración NOT NULL con backfill

**Decisión:** Nueva migración que:

1. Elimina filas con `id_estacion IS NULL` (si existen en entornos de prueba).
2. Aplica `ALTER COLUMN id_estacion SET NOT NULL`.

**Alternativa descartada:** Valor sentinel `0` — confunde con estación real.

En producción vacía o con datos válidos el paso 1 es no-op.

### 4. Política de error sin cambios

Errores de validación por `id_estacion` faltante → `ErrorDominio::CampoObligatorio` → ACK + log (comportamiento existente de `ack-validacion-error-08`).

## Risks / Trade-offs

- **[BREAKING]** Mensajes legacy sin `id_estacion` dejarán de procesarse → Mitigación: coordinar despliegue con Procesador; documentar en README.
- **[Datos históricos NULL]** Filas existentes sin estación se pierden en migración → Mitigación: solo entornos dev; en prod asumir datos completos o exportar antes.
- **[API usuario-viajes]** `idEstacionRetiro` puede seguir siendo nullable en respuesta si hubiera datos viejos → Mitigación: tras NOT NULL, retiros siempre tendrán estación; devoluciones pendientes mantienen null en campos de devolución.

## Migration Plan

1. Desplegar cambio de código (validación estricta).
2. Ejecutar migración SQL (`NOT NULL`).
3. Verificar smoke E2E con mensaje que incluya `id_estacion`.
4. Rollback: revertir migración (`DROP NOT NULL`) y código anterior si Procesador aún envía mensajes incompletos.

## Open Questions

- Ninguna bloqueante: el usuario confirmó obligatoriedad explícita (no vacío, nulo ni undefined).
