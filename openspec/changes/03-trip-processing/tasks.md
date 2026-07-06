## 1. Modelo de dominio

- [x] 1.1 Agregar `EstadoBicicletas` con `en_uso` y `maximo_historico`.
- [x] 1.2 Implementar calculo `bicicletas_disponibles = maximo_historico - en_uso`.
- [x] 1.3 Agregar tipos auxiliares para representar movimiento historico y retiro activo si hacen falta.

## 2. Errores de negocio

- [x] 2.1 Agregar `ErrorDominio::IdRecorridoYaUtilizado`.
- [x] 2.2 Agregar `ErrorDominio::RetiroConBicicletaEnUso`.
- [x] 2.3 Agregar `ErrorDominio::RetiroFueraDeOrdenTemporal`.
- [x] 2.4 Agregar `ErrorDominio::DevolucionSinRetiroPrevio`.
- [x] 2.5 Agregar `ErrorDominio::DevolucionConIdRecorridoDistinto`.
- [x] 2.6 Agregar `ErrorDominio::DevolucionConMismaFecha`.
- [x] 2.7 Agregar `ErrorDominio::DevolucionDuplicadaEnElTiempo`.

## 3. Puertos de dominio

- [x] 3.1 Definir trait `RepositorioRecorrido`.
- [x] 3.2 Agregar metodo para buscar si existe `id_recorrido`.
- [x] 3.3 Agregar metodo para buscar retiro activo del usuario.
- [x] 3.4 Agregar metodo para buscar ultimo movimiento antes de fecha.
- [x] 3.5 Agregar metodo para buscar siguiente movimiento despues de fecha.

## 4. SistemaBicicletas

- [x] 4.1 Crear `SistemaBicicletas` parametrizado por `RepositorioRecorrido`.
- [x] 4.2 Implementar procesamiento de retiro valido.
- [x] 4.3 Implementar validaciones de retiro duplicado, bicicleta en uso y orden temporal.
- [x] 4.4 Implementar procesamiento de devolucion valida.
- [x] 4.5 Implementar validaciones de devolucion sin retiro, id distinto, misma fecha y duplicada en el tiempo.

## 5. Tests unitarios

- [x] 5.1 Crear mock en memoria de `RepositorioRecorrido`.
- [x] 5.2 Testear retiro exitoso y actualizacion de `maximo_historico`.
- [x] 5.3 Testear rechazo de `id_recorrido` duplicado.
- [x] 5.4 Testear rechazo de segundo retiro sin devolucion.
- [x] 5.5 Testear rechazo de retiro fuera de orden temporal.
- [x] 5.6 Testear devolucion exitosa.
- [x] 5.7 Testear devolucion sin retiro previo.
- [x] 5.8 Testear devolucion con `id_recorrido` distinto.
- [x] 5.9 Testear devolucion con misma fecha que retiro.
- [x] 5.10 Testear devolucion duplicada en el tiempo.
- [x] 5.11 Testear calculo de bicicletas disponibles.

## 6. Verificacion

- [x] 6.1 Ejecutar `cargo fmt`.
- [x] 6.2 Ejecutar `cargo build`.
- [x] 6.3 Ejecutar `cargo test`.
- [x] 6.4 Validar el change con `openspec validate`.
