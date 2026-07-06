use anyhow::Context;
use dominio::EstadoBicicletas;
use sqlx::{Executor, PgConnection, PgPool, Postgres, Row};

pub async fn leer_estado(pool: &PgPool) -> anyhow::Result<EstadoBicicletas> {
    leer_estado_executor(pool).await
}

pub async fn leer_estado_conn(conn: &mut PgConnection) -> anyhow::Result<EstadoBicicletas> {
    leer_estado_executor(conn).await
}

pub async fn actualizar_estado(
    conn: &mut PgConnection,
    estado: EstadoBicicletas,
) -> anyhow::Result<()> {
    actualizar_estado_executor(conn, estado).await
}

pub async fn actualizar_estado_pool(pool: &PgPool, estado: EstadoBicicletas) -> anyhow::Result<()> {
    actualizar_estado_executor(pool, estado).await
}

async fn leer_estado_executor<'e, E>(executor: E) -> anyhow::Result<EstadoBicicletas>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query(
        r#"
        SELECT en_uso, maximo_historico
        FROM estado_bicicletas
        WHERE id = 1
        "#,
    )
    .fetch_one(executor)
    .await
    .context("no se pudo leer estado_bicicletas")?;

    let en_uso: i32 = row.try_get("en_uso")?;
    let maximo_historico: i32 = row.try_get("maximo_historico")?;

    Ok(EstadoBicicletas::new(en_uso as u64, maximo_historico as u64))
}

async fn actualizar_estado_executor<'e, E>(
    executor: E,
    estado: EstadoBicicletas,
) -> anyhow::Result<()>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        r#"
        UPDATE estado_bicicletas
        SET en_uso = $1,
            maximo_historico = $2,
            updated_at = NOW()
        WHERE id = 1
        "#,
    )
    .bind(estado.en_uso as i32)
    .bind(estado.maximo_historico as i32)
    .execute(executor)
    .await
    .context("no se pudo actualizar estado_bicicletas")?;

    Ok(())
}
