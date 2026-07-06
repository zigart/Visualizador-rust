use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ErrorDominio {
    #[error("campo obligatorio ausente o vacio: {campo}")]
    CampoObligatorio { campo: &'static str },

    #[error("valor invalido para {campo}: {valor}")]
    ValorInvalido { campo: &'static str, valor: String },

    #[error("payload JSON invalido: {detalle}")]
    JsonInvalido { detalle: String },

    #[error("operacion de dominio no implementada en el bootstrap")]
    NoImplementado,
}
