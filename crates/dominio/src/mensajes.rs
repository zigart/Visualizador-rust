use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operacion {
    Retiro,
    Devolucion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovimientoRecorrido {
    pub id_recorrido: u64,
    pub id_usuario: u64,
    pub id_estacion: Option<u64>,
    pub operacion: Operacion,
    pub fechahora: DateTime<Utc>,
}
