## 1. Contrato y migracion

- [x] 1.1 Agregar migracion `ADD COLUMN id_estacion` a `recorridos`.
- [x] 1.2 Actualizar modelo de movimiento para incluir `id_estacion` cuando exista.
- [x] 1.3 Actualizar parser de mensajes para aceptar `id_estacion`.
- [x] 1.4 Definir si `id_estacion` es obligatorio u opcional segun contrato con Procesador.

## 2. ConsultaRepositorioRecorrido

- [x] 2.1 Definir trait o metodo `listar_viajes_por_usuario`.
- [x] 2.2 Implementar consulta SQL para movimientos del usuario.
- [x] 2.3 Ordenar por `fechaHoraDevolucion ASC NULLS LAST`.
- [x] 2.4 Retornar coleccion vacia cuando no hay movimientos.

## 3. Servicio de viajes

- [x] 3.1 Definir DTO `Viaje` con `idRecorrido`, `fechaHoraRetiro`, `idEstacionRetiro`, `fechaHoraDevolucion`, `idEstacionDevolucion`.
- [x] 3.2 Agrupar retiro y devolucion por `id_recorrido`.
- [x] 3.3 Representar devolucion pendiente con campos `null`.
- [x] 3.4 Mantener orden ascendente por devolucion con pendientes al final.

## 4. Handler HTTP

- [x] 4.1 Crear handler `GET /usuarios/:id_usuario`.
- [x] 4.2 Responder 200 con `{ "viajes": [...] }` cuando existan viajes.
- [x] 4.3 Responder 404 cuando no existan movimientos para el usuario.
- [x] 4.4 Registrar ruta en `axum`.

## 5. Tests de aceptacion

- [x] 5.1 Testear caso feliz con viajes ordenados.
- [x] 5.2 Testear viaje en curso con campos de devolucion `null`.
- [x] 5.3 Testear 404 para usuario inexistente.
- [x] 5.4 Testear estructura de campos camelCase.

## 6. Verificacion

- [x] 6.1 Ejecutar `cargo fmt`.
- [x] 6.2 Ejecutar `cargo build`.
- [x] 6.3 Ejecutar `cargo test`.
- [x] 6.4 Ejecutar `sqlx migrate run`.
- [x] 6.5 Validar el change con `openspec validate`.
