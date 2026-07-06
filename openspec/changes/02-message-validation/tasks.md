## 1. Errores de dominio

- [ ] 1.1 Agregar `ErrorDominio::CampoObligatorio` con nombre de campo.
- [ ] 1.2 Agregar `ErrorDominio::ValorInvalido` con nombre de campo y valor recibido.
- [ ] 1.3 Agregar error de dominio para JSON invalido o formato invalido.

## 2. Modelo de mensaje

- [ ] 2.1 Definir enum de operacion con variantes `Retiro` y `Devolucion`.
- [ ] 2.2 Definir struct validada para mensaje entrante con `id_recorrido`, `id_usuario`, `operacion` y `fechahora`.
- [ ] 2.3 Definir DTO de deserializacion flexible para IDs como numero o string numerico.

## 3. Parser y validacion

- [ ] 3.1 Implementar parser de payload JSON con objeto raiz obligatorio.
- [ ] 3.2 Validar campos obligatorios no vacios.
- [ ] 3.3 Validar operacion permitida.
- [ ] 3.4 Parsear `fechahora` como ISO 8601.
- [ ] 3.5 Coercionar IDs string numericos antes de validar.

## 4. Integracion con consumer

- [ ] 4.1 Exponer funcion de parseo desde `adaptadores` o modulo compartido apropiado.
- [ ] 4.2 Loguear errores de parseo y validacion.
- [ ] 4.3 Conectar errores de validacion con la decision NACK del consumer.

## 5. Tests

- [ ] 5.1 Agregar test para payload no JSON.
- [ ] 5.2 Agregar test para campo obligatorio faltante.
- [ ] 5.3 Agregar test para operacion invalida.
- [ ] 5.4 Agregar test para fecha vacia o no parseable.
- [ ] 5.5 Agregar test para IDs como string numerico.
- [ ] 5.6 Ejecutar `cargo fmt`, `cargo build`, `cargo test` y `openspec validate`.
