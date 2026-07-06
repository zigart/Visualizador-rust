use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ErrorDominio {
    #[error("campo obligatorio ausente o vacio: {campo}")]
    CampoObligatorio { campo: &'static str },

    #[error("valor invalido para {campo}: {valor}")]
    ValorInvalido { campo: &'static str, valor: String },

    #[error("payload JSON invalido: {detalle}")]
    JsonInvalido { detalle: String },

    #[error("id_recorrido ya utilizado: {id_recorrido}")]
    IdRecorridoYaUtilizado { id_recorrido: u64 },

    #[error("el usuario ya tiene una bicicleta en uso: {id_usuario}")]
    RetiroConBicicletaEnUso { id_usuario: u64 },

    #[error("retiro fuera de orden temporal para usuario: {id_usuario}")]
    RetiroFueraDeOrdenTemporal { id_usuario: u64 },

    #[error("devolucion sin retiro previo para usuario: {id_usuario}")]
    DevolucionSinRetiroPrevio { id_usuario: u64 },

    #[error("devolucion con id_recorrido distinto: esperado {esperado}, recibido {recibido}")]
    DevolucionConIdRecorridoDistinto { esperado: u64, recibido: u64 },

    #[error("devolucion con la misma fecha que el retiro para id_recorrido: {id_recorrido}")]
    DevolucionConMismaFecha { id_recorrido: u64 },

    #[error("devolucion duplicada en el tiempo para usuario: {id_usuario}")]
    DevolucionDuplicadaEnElTiempo { id_usuario: u64 },

    #[error("operacion de dominio no implementada en el bootstrap")]
    NoImplementado,
}
