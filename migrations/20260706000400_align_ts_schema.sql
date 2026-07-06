CREATE TABLE recorridos_new (
    id SERIAL PRIMARY KEY,
    id_recorrido INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    operacion VARCHAR(20) NOT NULL,
    fechahora TIMESTAMPTZ NOT NULL,
    id_estacion BIGINT
);

CREATE TABLE estado_bicicletas_new (
    id SERIAL PRIMARY KEY,
    en_uso INTEGER NOT NULL DEFAULT 0,
    maximo_historico INTEGER NOT NULL DEFAULT 0
);

INSERT INTO recorridos_new (id_recorrido, id_usuario, operacion, fechahora, id_estacion)
SELECT
    id_recorrido::INTEGER,
    id_usuario::INTEGER,
    operacion,
    fechahora,
    id_estacion
FROM recorridos
WHERE id_recorrido IS NOT NULL
  AND id_usuario IS NOT NULL
  AND operacion IS NOT NULL
  AND fechahora IS NOT NULL;

INSERT INTO estado_bicicletas_new (id, en_uso, maximo_historico)
SELECT id::INTEGER, en_uso, maximo_historico
FROM estado_bicicletas;

INSERT INTO estado_bicicletas_new (en_uso, maximo_historico)
SELECT 0, 0
WHERE NOT EXISTS (SELECT 1 FROM estado_bicicletas_new);

SELECT setval(
    pg_get_serial_sequence('recorridos_new', 'id'),
    COALESCE((SELECT MAX(id) FROM recorridos_new), 1)
);

SELECT setval(
    pg_get_serial_sequence('estado_bicicletas_new', 'id'),
    COALESCE((SELECT MAX(id) FROM estado_bicicletas_new), 1)
);

DROP TABLE recorridos;
DROP TABLE estado_bicicletas;

ALTER TABLE recorridos_new RENAME TO recorridos;
ALTER TABLE estado_bicicletas_new RENAME TO estado_bicicletas;

CREATE INDEX recorridos_usuario_fechahora_idx
    ON recorridos (id_usuario, fechahora);

CREATE INDEX recorridos_usuario_devolucion_idx
    ON recorridos (id_usuario, fechahora)
    WHERE operacion = 'devolucion';

CREATE UNIQUE INDEX recorridos_id_recorrido_operacion_idx
    ON recorridos (id_recorrido, operacion);
