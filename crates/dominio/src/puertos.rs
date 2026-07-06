use chrono::{DateTime, Utc};

use crate::MovimientoRecorrido;

pub trait RepositorioRecorrido {
    fn existe_id_recorrido(&self, id_recorrido: u64) -> bool;

    fn retiro_activo_usuario(
        &self,
        id_usuario: u64,
        fechahora: DateTime<Utc>,
    ) -> Option<MovimientoRecorrido>;

    fn ultimo_movimiento_antes(
        &self,
        id_usuario: u64,
        fechahora: DateTime<Utc>,
    ) -> Option<MovimientoRecorrido>;

    fn siguiente_movimiento_despues(
        &self,
        id_usuario: u64,
        fechahora: DateTime<Utc>,
    ) -> Option<MovimientoRecorrido>;
}
