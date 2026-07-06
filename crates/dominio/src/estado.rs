#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EstadoBicicletas {
    pub en_uso: u64,
    pub maximo_historico: u64,
}

impl EstadoBicicletas {
    pub fn new(en_uso: u64, maximo_historico: u64) -> Self {
        Self {
            en_uso,
            maximo_historico,
        }
    }

    pub fn bicicletas_disponibles(&self) -> u64 {
        self.maximo_historico.saturating_sub(self.en_uso)
    }

    pub fn registrar_retiro(self) -> Self {
        let en_uso = self.en_uso + 1;

        Self {
            en_uso,
            maximo_historico: self.maximo_historico.max(en_uso),
        }
    }

    pub fn registrar_devolucion(self) -> Self {
        Self {
            en_uso: self.en_uso.saturating_sub(1),
            maximo_historico: self.maximo_historico,
        }
    }
}
