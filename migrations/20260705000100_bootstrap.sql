CREATE TABLE recorridos (
    id BIGSERIAL PRIMARY KEY,
    bicicleta_id TEXT NOT NULL,
    estacion_origen_id TEXT NOT NULL,
    estacion_destino_id TEXT,
    iniciado_en TIMESTAMPTZ NOT NULL,
    finalizado_en TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT recorridos_fin_posterior_inicio CHECK (
        finalizado_en IS NULL OR finalizado_en >= iniciado_en
    )
);

CREATE UNIQUE INDEX recorridos_bicicleta_activa_idx
    ON recorridos (bicicleta_id)
    WHERE finalizado_en IS NULL;

CREATE TABLE estado_bicicletas (
    id SMALLINT PRIMARY KEY DEFAULT 1,
    bicicletas_disponibles INTEGER NOT NULL DEFAULT 0,
    bicicletas_retiradas INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT estado_bicicletas_singleton CHECK (id = 1),
    CONSTRAINT estado_bicicletas_disponibles_no_negativo CHECK (bicicletas_disponibles >= 0),
    CONSTRAINT estado_bicicletas_retiradas_no_negativo CHECK (bicicletas_retiradas >= 0)
);

INSERT INTO estado_bicicletas (id, bicicletas_disponibles, bicicletas_retiradas)
VALUES (1, 0, 0);
