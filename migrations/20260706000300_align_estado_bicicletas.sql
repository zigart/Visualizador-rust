ALTER TABLE estado_bicicletas
    RENAME COLUMN bicicletas_retiradas TO en_uso;

ALTER TABLE estado_bicicletas
    RENAME COLUMN bicicletas_disponibles TO maximo_historico;

ALTER TABLE estado_bicicletas
    RENAME CONSTRAINT estado_bicicletas_retiradas_no_negativo TO estado_bicicletas_en_uso_no_negativo;

ALTER TABLE estado_bicicletas
    RENAME CONSTRAINT estado_bicicletas_disponibles_no_negativo TO estado_bicicletas_maximo_historico_no_negativo;

UPDATE estado_bicicletas
SET maximo_historico = GREATEST(maximo_historico, en_uso)
WHERE id = 1;

CREATE UNIQUE INDEX IF NOT EXISTS recorridos_id_recorrido_operacion_idx
    ON recorridos (id_recorrido, operacion)
    WHERE id_recorrido IS NOT NULL AND operacion IS NOT NULL;
