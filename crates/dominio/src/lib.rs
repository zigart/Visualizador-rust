#![forbid(unsafe_code)]

pub mod errores;
pub mod mensajes;

pub use errores::ErrorDominio;
pub use mensajes::{MovimientoRecorrido, Operacion};
