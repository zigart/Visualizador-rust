ALTER TABLE recorridos
ADD COLUMN IF NOT EXISTS id_recorrido BIGINT,
ADD COLUMN IF NOT EXISTS id_usuario BIGINT,
ADD COLUMN IF NOT EXISTS operacion TEXT,
ADD COLUMN IF NOT EXISTS fechahora TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS recorridos_usuario_fechahora_idx
    ON recorridos (id_usuario, fechahora);

CREATE INDEX IF NOT EXISTS recorridos_usuario_devolucion_idx
    ON recorridos (id_usuario, fechahora)
    WHERE operacion = 'devolucion';
