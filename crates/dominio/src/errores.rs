use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorDominio {
    #[error("operacion de dominio no implementada en el bootstrap")]
    NoImplementado,
}
