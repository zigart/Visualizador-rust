DELETE FROM recorridos WHERE id_estacion IS NULL;

ALTER TABLE recorridos
    ALTER COLUMN id_estacion SET NOT NULL;
