use chrono::{DateTime, Utc};
use dominio::{ErrorDominio, MovimientoRecorrido, Operacion};
use serde_json::Value;

pub fn parse_movimiento_payload(payload: &[u8]) -> Result<MovimientoRecorrido, ErrorDominio> {
    let value: Value =
        serde_json::from_slice(payload).map_err(|error| ErrorDominio::JsonInvalido {
            detalle: error.to_string(),
        })?;

    let object = value
        .as_object()
        .ok_or_else(|| ErrorDominio::JsonInvalido {
            detalle: "el payload debe ser un objeto JSON".to_string(),
        })?;

    let id_recorrido = parse_required_id(object.get("id_recorrido"), "id_recorrido")?;
    let id_usuario = parse_required_id(object.get("id_usuario"), "id_usuario")?;
    let id_estacion = parse_optional_id(object.get("id_estacion"), "id_estacion")?;
    let operacion = parse_operacion(required_non_empty_str(
        object.get("operacion"),
        "operacion",
    )?)?;
    let fechahora = parse_fechahora(required_non_empty_str(
        object.get("fechahora"),
        "fechahora",
    )?)?;

    Ok(MovimientoRecorrido {
        id_recorrido,
        id_usuario,
        id_estacion,
        operacion,
        fechahora,
    })
}

fn parse_required_id(value: Option<&Value>, campo: &'static str) -> Result<u64, ErrorDominio> {
    match value {
        None | Some(Value::Null) => Err(ErrorDominio::CampoObligatorio { campo }),
        Some(Value::Number(number)) => number.as_u64().ok_or_else(|| ErrorDominio::ValorInvalido {
            campo,
            valor: number.to_string(),
        }),
        Some(Value::String(text)) if text.trim().is_empty() => {
            Err(ErrorDominio::CampoObligatorio { campo })
        }
        Some(Value::String(text)) => text
            .parse::<u64>()
            .map_err(|_| ErrorDominio::ValorInvalido {
                campo,
                valor: text.clone(),
            }),
        Some(other) => Err(ErrorDominio::ValorInvalido {
            campo,
            valor: other.to_string(),
        }),
    }
}

fn parse_optional_id(
    value: Option<&Value>,
    campo: &'static str,
) -> Result<Option<u64>, ErrorDominio> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) if text.trim().is_empty() => Ok(None),
        Some(_) => parse_required_id(value, campo).map(Some),
    }
}

fn required_non_empty_str(
    value: Option<&Value>,
    campo: &'static str,
) -> Result<String, ErrorDominio> {
    match value {
        None | Some(Value::Null) => Err(ErrorDominio::CampoObligatorio { campo }),
        Some(Value::String(text)) if text.trim().is_empty() => {
            Err(ErrorDominio::CampoObligatorio { campo })
        }
        Some(Value::String(text)) => Ok(text.clone()),
        Some(other) => Err(ErrorDominio::ValorInvalido {
            campo,
            valor: other.to_string(),
        }),
    }
}

fn parse_operacion(value: String) -> Result<Operacion, ErrorDominio> {
    match value.as_str() {
        "retiro" => Ok(Operacion::Retiro),
        "devolucion" => Ok(Operacion::Devolucion),
        _ => Err(ErrorDominio::ValorInvalido {
            campo: "operacion",
            valor: value,
        }),
    }
}

fn parse_fechahora(value: String) -> Result<DateTime<Utc>, ErrorDominio> {
    DateTime::parse_from_rfc3339(&value)
        .map(|date| date.with_timezone(&Utc))
        .map_err(|_| ErrorDominio::ValorInvalido {
            campo: "fechahora",
            valor: value,
        })
}

#[cfg(test)]
mod tests {
    use dominio::{ErrorDominio, Operacion};

    use super::parse_movimiento_payload;

    #[test]
    fn rechaza_payload_no_json() {
        let error = parse_movimiento_payload(b"no-json").unwrap_err();

        assert!(matches!(error, ErrorDominio::JsonInvalido { .. }));
    }

    #[test]
    fn rechaza_json_sin_objeto_raiz() {
        let error = parse_movimiento_payload(br#"[1,2,3]"#).unwrap_err();

        assert!(matches!(error, ErrorDominio::JsonInvalido { .. }));
    }

    #[test]
    fn rechaza_campo_obligatorio_faltante() {
        let error = parse_movimiento_payload(
            br#"{"id_recorrido":1,"id_usuario":1,"fechahora":"2026-06-08T15:34:20Z"}"#,
        )
        .unwrap_err();

        assert_eq!(error, ErrorDominio::CampoObligatorio { campo: "operacion" });
    }

    #[test]
    fn rechaza_operacion_invalida() {
        let error = parse_movimiento_payload(
            br#"{"id_recorrido":1,"id_usuario":1,"operacion":"X","fechahora":"2026-06-08T15:34:20Z"}"#,
        )
        .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::ValorInvalido {
                campo: "operacion",
                valor: "X".to_string(),
            }
        );
    }

    #[test]
    fn rechaza_fecha_vacia() {
        let error = parse_movimiento_payload(
            br#"{"id_recorrido":1,"id_usuario":1,"operacion":"retiro","fechahora":""}"#,
        )
        .unwrap_err();

        assert_eq!(error, ErrorDominio::CampoObligatorio { campo: "fechahora" });
    }

    #[test]
    fn rechaza_fecha_no_parseable() {
        let error = parse_movimiento_payload(
            br#"{"id_recorrido":1,"id_usuario":1,"operacion":"retiro","fechahora":"ayer"}"#,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            ErrorDominio::ValorInvalido {
                campo: "fechahora",
                ..
            }
        ));
    }

    #[test]
    fn acepta_ids_como_string_numerico() {
        let movimiento = parse_movimiento_payload(
            br#"{"id_recorrido":"880001","id_usuario":"42","operacion":"retiro","fechahora":"2026-06-08T15:34:20Z"}"#,
        )
        .unwrap();

        assert_eq!(movimiento.id_recorrido, 880001);
        assert_eq!(movimiento.id_usuario, 42);
        assert_eq!(movimiento.operacion, Operacion::Retiro);
    }

    #[test]
    fn acepta_id_estacion_opcional() {
        let movimiento = parse_movimiento_payload(
            br#"{"id_recorrido":1,"id_usuario":1,"id_estacion":"15","operacion":"retiro","fechahora":"2026-06-08T15:34:20Z"}"#,
        )
        .unwrap();

        assert_eq!(movimiento.id_estacion, Some(15));
    }
}
