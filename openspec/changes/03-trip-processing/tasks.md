## 1. Modelo de dominio

- [ ] 1.1 Agregar `EstadoBicicletas` con `en_uso` y `maximo_historico`.
- [ ] 1.2 Implementar calculo `bicicletas_disponibles = maximo_historico - en_uso`.
- [ ] 1.3 Agregar tipos auxiliares para representar movimiento historico y retiro activo si hacen falta.

## 2. Errores de negocio

- [ ] 2.1 Agregar `ErrorDominio::IdRecorridoYaUtilizado`.
- [ ] 2.2 Agregar `ErrorDominio::RetiroConBicicletaEnUso`.
- [ ] 2.3 Agregar `ErrorDominio::RetiroFueraDeOrdenTemporal`.
- [ ] 2.4 Agregar `ErrorDominio::DevolucionSinRetiroPrevio`.
- [ ] 2.5 Agregar `ErrorDominio::DevolucionConIdRecorridoDistinto`.
- [ ] 2.6 Agregar `ErrorDominio::DevolucionConMismaFecha`.
- [ ] 2.7 Agregar `ErrorDominio::DevolucionDuplicadaEnElTiempo`.

## 3. Puertos de dominio

- [ ] 3.1 Definir trait `RepositorioRecorrido`.
- [ ] 3.2 Agregar metodo para buscar si existe `id_recorrido`.
- [ ] 3.3 Agregar metodo para buscar retiro activo del usuario.
- [ ] 3.4 Agregar metodo para buscar ultimo movimiento antes de fecha.
- [ ] 3.5 Agregar metodo para buscar siguiente movimiento despues de fecha.

## 4. SistemaBicicletas

- [ ] 4.1 Crear `SistemaBicicletas` parametrizado por `RepositorioRecorrido`.
- [ ] 4.2 Implementar procesamiento de retiro valido.
- [ ] 4.3 Implementar validaciones de retiro duplicado, bicicleta en uso y orden temporal.
- [ ] 4.4 Implementar procesamiento de devolucion valida.
- [ ] 4.5 Implementar validaciones de devolucion sin retiro, id distinto, misma fecha y duplicada en el tiempo.

## 5. Tests unitarios

- [ ] 5.1 Crear mock en memoria de `RepositorioRecorrido`.
- [ ] 5.2 Testear retiro exitoso y actualizacion de `maximo_historico`.
- [ ] 5.3 Testear rechazo de `id_recorrido` duplicado.
- [ ] 5.4 Testear rechazo de segundo retiro sin devolucion.
- [ ] 5.5 Testear rechazo de retiro fuera de orden temporal.
- [ ] 5.6 Testear devolucion exitosa.
- [ ] 5.7 Testear devolucion sin retiro previo.
- [ ] 5.8 Testear devolucion con `id_recorrido` distinto.
- [ ] 5.9 Testear devolucion con misma fecha que retiro.
- [ ] 5.10 Testear devolucion duplicada en el tiempo.
- [ ] 5.11 Testear calculo de bicicletas disponibles.

## 6. Verificacion

- [ ] 6.1 Ejecutar `cargo fmt`.
- [ ] 6.2 Ejecutar `cargo build`.
- [ ] 6.3 Ejecutar `cargo test`.
- [ ] 6.4 Validar el change con `openspec validate`.
