use crate::{ErrorDominio, EstadoBicicletas, MovimientoRecorrido, Operacion, RepositorioRecorrido};

pub struct SistemaBicicletas<R> {
    repositorio: R,
}

impl<R> SistemaBicicletas<R>
where
    R: RepositorioRecorrido,
{
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub fn procesar(
        &self,
        movimiento: &MovimientoRecorrido,
        estado: EstadoBicicletas,
    ) -> Result<EstadoBicicletas, ErrorDominio> {
        match movimiento.operacion {
            Operacion::Retiro => self.procesar_retiro(movimiento, estado),
            Operacion::Devolucion => self.procesar_devolucion(movimiento, estado),
        }
    }

    fn procesar_retiro(
        &self,
        movimiento: &MovimientoRecorrido,
        estado: EstadoBicicletas,
    ) -> Result<EstadoBicicletas, ErrorDominio> {
        if self
            .repositorio
            .existe_id_recorrido(movimiento.id_recorrido)
        {
            return Err(ErrorDominio::IdRecorridoYaUtilizado {
                id_recorrido: movimiento.id_recorrido,
            });
        }

        if self
            .repositorio
            .retiro_activo_usuario(movimiento.id_usuario, movimiento.fechahora)
            .is_some()
        {
            return Err(ErrorDominio::RetiroConBicicletaEnUso {
                id_usuario: movimiento.id_usuario,
            });
        }

        if matches!(
            self.repositorio
                .siguiente_movimiento_despues(movimiento.id_usuario, movimiento.fechahora),
            Some(MovimientoRecorrido {
                operacion: Operacion::Retiro,
                ..
            })
        ) {
            return Err(ErrorDominio::RetiroFueraDeOrdenTemporal {
                id_usuario: movimiento.id_usuario,
            });
        }

        Ok(estado.registrar_retiro())
    }

    fn procesar_devolucion(
        &self,
        movimiento: &MovimientoRecorrido,
        estado: EstadoBicicletas,
    ) -> Result<EstadoBicicletas, ErrorDominio> {
        let Some(retiro) = self
            .repositorio
            .retiro_activo_usuario(movimiento.id_usuario, movimiento.fechahora)
        else {
            return Err(ErrorDominio::DevolucionSinRetiroPrevio {
                id_usuario: movimiento.id_usuario,
            });
        };

        if retiro.id_recorrido != movimiento.id_recorrido {
            return Err(ErrorDominio::DevolucionConIdRecorridoDistinto {
                esperado: retiro.id_recorrido,
                recibido: movimiento.id_recorrido,
            });
        }

        if retiro.fechahora == movimiento.fechahora {
            return Err(ErrorDominio::DevolucionConMismaFecha {
                id_recorrido: movimiento.id_recorrido,
            });
        }

        if matches!(
            self.repositorio
                .siguiente_movimiento_despues(movimiento.id_usuario, movimiento.fechahora),
            Some(MovimientoRecorrido {
                operacion: Operacion::Devolucion,
                ..
            })
        ) {
            return Err(ErrorDominio::DevolucionDuplicadaEnElTiempo {
                id_usuario: movimiento.id_usuario,
            });
        }

        Ok(estado.registrar_devolucion())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::SistemaBicicletas;
    use crate::{
        ErrorDominio, EstadoBicicletas, MovimientoRecorrido, Operacion, RepositorioRecorrido,
    };

    #[derive(Default)]
    struct RepositorioMock {
        movimientos: Vec<MovimientoRecorrido>,
    }

    impl RepositorioMock {
        fn con_movimientos(movimientos: Vec<MovimientoRecorrido>) -> Self {
            Self { movimientos }
        }
    }

    impl RepositorioRecorrido for RepositorioMock {
        fn existe_id_recorrido(&self, id_recorrido: u64) -> bool {
            self.movimientos
                .iter()
                .any(|movimiento| movimiento.id_recorrido == id_recorrido)
        }

        fn retiro_activo_usuario(
            &self,
            id_usuario: u64,
            fechahora: chrono::DateTime<Utc>,
        ) -> Option<MovimientoRecorrido> {
            match self.ultimo_movimiento_antes(id_usuario, fechahora) {
                Some(movimiento) if movimiento.operacion == Operacion::Retiro => Some(movimiento),
                _ => None,
            }
        }

        fn ultimo_movimiento_antes(
            &self,
            id_usuario: u64,
            fechahora: chrono::DateTime<Utc>,
        ) -> Option<MovimientoRecorrido> {
            self.movimientos
                .iter()
                .filter(|movimiento| {
                    movimiento.id_usuario == id_usuario && movimiento.fechahora <= fechahora
                })
                .max_by_key(|movimiento| (movimiento.fechahora, movimiento.id_recorrido))
                .cloned()
        }

        fn siguiente_movimiento_despues(
            &self,
            id_usuario: u64,
            fechahora: chrono::DateTime<Utc>,
        ) -> Option<MovimientoRecorrido> {
            self.movimientos
                .iter()
                .filter(|movimiento| {
                    movimiento.id_usuario == id_usuario && movimiento.fechahora > fechahora
                })
                .min_by_key(|movimiento| (movimiento.fechahora, movimiento.id_recorrido))
                .cloned()
        }
    }

    #[test]
    fn retiro_exitoso_incrementa_en_uso_y_actualiza_maximo_historico() {
        let sistema = SistemaBicicletas::new(RepositorioMock::default());
        let estado = EstadoBicicletas::new(4, 4);

        let resultado = sistema
            .procesar(&retiro(1, 10, 10), estado)
            .expect("retiro valido");

        assert_eq!(resultado.en_uso, 5);
        assert_eq!(resultado.maximo_historico, 5);
    }

    #[test]
    fn retiro_rechaza_id_recorrido_duplicado() {
        let sistema =
            SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![retiro(1, 10, 1)]));

        let error = sistema
            .procesar(&retiro(1, 20, 2), EstadoBicicletas::new(0, 0))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::IdRecorridoYaUtilizado { id_recorrido: 1 }
        );
    }

    #[test]
    fn retiro_rechaza_segundo_retiro_sin_devolucion() {
        let sistema =
            SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![retiro(1, 10, 1)]));

        let error = sistema
            .procesar(&retiro(2, 10, 2), EstadoBicicletas::new(1, 1))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::RetiroConBicicletaEnUso { id_usuario: 10 }
        );
    }

    #[test]
    fn retiro_rechaza_fuera_de_orden_temporal_si_existe_retiro_posterior() {
        let sistema =
            SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![retiro(2, 10, 5)]));

        let error = sistema
            .procesar(&retiro(1, 10, 3), EstadoBicicletas::new(0, 0))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::RetiroFueraDeOrdenTemporal { id_usuario: 10 }
        );
    }

    #[test]
    fn devolucion_exitosa_decrementa_en_uso() {
        let sistema =
            SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![retiro(1, 10, 1)]));

        let resultado = sistema
            .procesar(&devolucion(1, 10, 2), EstadoBicicletas::new(3, 5))
            .expect("devolucion valida");

        assert_eq!(resultado.en_uso, 2);
        assert_eq!(resultado.maximo_historico, 5);
    }

    #[test]
    fn devolucion_rechaza_sin_retiro_previo() {
        let sistema = SistemaBicicletas::new(RepositorioMock::default());

        let error = sistema
            .procesar(&devolucion(1, 10, 2), EstadoBicicletas::new(0, 0))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::DevolucionSinRetiroPrevio { id_usuario: 10 }
        );
    }

    #[test]
    fn devolucion_rechaza_id_recorrido_distinto_al_retiro_activo() {
        let sistema =
            SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![retiro(1, 10, 1)]));

        let error = sistema
            .procesar(&devolucion(2, 10, 2), EstadoBicicletas::new(1, 1))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::DevolucionConIdRecorridoDistinto {
                esperado: 1,
                recibido: 2,
            }
        );
    }

    #[test]
    fn devolucion_rechaza_misma_fecha_que_retiro() {
        let sistema =
            SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![retiro(1, 10, 1)]));

        let error = sistema
            .procesar(&devolucion(1, 10, 1), EstadoBicicletas::new(1, 1))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::DevolucionConMismaFecha { id_recorrido: 1 }
        );
    }

    #[test]
    fn devolucion_rechaza_duplicada_en_el_tiempo_si_existe_devolucion_posterior() {
        let sistema = SistemaBicicletas::new(RepositorioMock::con_movimientos(vec![
            retiro(1, 10, 1),
            devolucion(1, 10, 5),
        ]));

        let error = sistema
            .procesar(&devolucion(1, 10, 3), EstadoBicicletas::new(1, 1))
            .unwrap_err();

        assert_eq!(
            error,
            ErrorDominio::DevolucionDuplicadaEnElTiempo { id_usuario: 10 }
        );
    }

    #[test]
    fn calcula_bicicletas_disponibles() {
        let estado = EstadoBicicletas::new(2, 5);

        assert_eq!(estado.bicicletas_disponibles(), 3);
    }

    fn retiro(id_recorrido: u64, id_usuario: u64, dia: u32) -> MovimientoRecorrido {
        movimiento(id_recorrido, id_usuario, Operacion::Retiro, dia)
    }

    fn devolucion(id_recorrido: u64, id_usuario: u64, dia: u32) -> MovimientoRecorrido {
        movimiento(id_recorrido, id_usuario, Operacion::Devolucion, dia)
    }

    fn movimiento(
        id_recorrido: u64,
        id_usuario: u64,
        operacion: Operacion,
        dia: u32,
    ) -> MovimientoRecorrido {
        MovimientoRecorrido {
            id_recorrido,
            id_usuario,
            operacion,
            fechahora: Utc.with_ymd_and_hms(2026, 7, dia, 12, 0, 0).unwrap(),
        }
    }
}
