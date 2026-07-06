use anyhow::Context;
use chrono::{DateTime, Utc};
use dominio::{MovimientoRecorrido, Operacion};
use sqlx::{postgres::PgRow, Executor, PgConnection, PgPool, Postgres, Row};

pub async fn existe_id_recorrido(
    conn: &mut PgConnection,
    id_recorrido: u64,
) -> anyhow::Result<bool> {
    existe_id_recorrido_executor(conn, id_recorrido).await
}

pub async fn listar_movimientos_usuario(
    pool: &PgPool,
    id_usuario: u64,
) -> anyhow::Result<Vec<MovimientoRecorrido>> {
    listar_movimientos_usuario_executor(pool, id_usuario).await
}

pub async fn listar_movimientos_usuario_conn(
    conn: &mut PgConnection,
    id_usuario: u64,
) -> anyhow::Result<Vec<MovimientoRecorrido>> {
    listar_movimientos_usuario_executor(conn, id_usuario).await
}

pub async fn ultimo_movimiento_antes(
    pool: &PgPool,
    id_usuario: u64,
    fechahora: DateTime<Utc>,
) -> anyhow::Result<Option<MovimientoRecorrido>> {
    ultimo_movimiento_antes_executor(pool, id_usuario, fechahora).await
}

pub async fn siguiente_movimiento_despues(
    pool: &PgPool,
    id_usuario: u64,
    fechahora: DateTime<Utc>,
) -> anyhow::Result<Option<MovimientoRecorrido>> {
    siguiente_movimiento_despues_executor(pool, id_usuario, fechahora).await
}

pub async fn insertar_movimiento(
    conn: &mut PgConnection,
    movimiento: &MovimientoRecorrido,
) -> anyhow::Result<()> {
    insertar_movimiento_executor(conn, movimiento).await
}

async fn existe_id_recorrido_executor<'e, E>(executor: E, id_recorrido: u64) -> anyhow::Result<bool>
where
    E: Executor<'e, Database = Postgres>,
{
    let existe = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM recorridos
            WHERE id_recorrido = $1
        )
        "#,
    )
    .bind(id_recorrido as i64)
    .fetch_one(executor)
    .await
    .context("no se pudo verificar id_recorrido")?;

    Ok(existe)
}

async fn listar_movimientos_usuario_executor<'e, E>(
    executor: E,
    id_usuario: u64,
) -> anyhow::Result<Vec<MovimientoRecorrido>>
where
    E: Executor<'e, Database = Postgres>,
{
    let rows = sqlx::query(
        r#"
        SELECT id_recorrido, id_usuario, id_estacion, operacion, fechahora
        FROM recorridos
        WHERE id_usuario = $1
          AND id_recorrido IS NOT NULL
          AND operacion IS NOT NULL
          AND fechahora IS NOT NULL
        ORDER BY fechahora ASC, id_recorrido ASC
        "#,
    )
    .bind(id_usuario as i64)
    .fetch_all(executor)
    .await
    .context("no se pudieron listar movimientos del usuario")?;

    rows.into_iter().map(movimiento_desde_fila).collect()
}

async fn ultimo_movimiento_antes_executor<'e, E>(
    executor: E,
    id_usuario: u64,
    fechahora: DateTime<Utc>,
) -> anyhow::Result<Option<MovimientoRecorrido>>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query(
        r#"
        SELECT id_recorrido, id_usuario, id_estacion, operacion, fechahora
        FROM recorridos
        WHERE id_usuario = $1
          AND fechahora <= $2
          AND id_recorrido IS NOT NULL
          AND operacion IS NOT NULL
          AND fechahora IS NOT NULL
        ORDER BY fechahora DESC, id_recorrido DESC
        LIMIT 1
        "#,
    )
    .bind(id_usuario as i64)
    .bind(fechahora)
    .fetch_optional(executor)
    .await
    .context("no se pudo consultar ultimo movimiento antes de fecha")?;

    row.map(movimiento_desde_fila).transpose()
}

async fn siguiente_movimiento_despues_executor<'e, E>(
    executor: E,
    id_usuario: u64,
    fechahora: DateTime<Utc>,
) -> anyhow::Result<Option<MovimientoRecorrido>>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query(
        r#"
        SELECT id_recorrido, id_usuario, id_estacion, operacion, fechahora
        FROM recorridos
        WHERE id_usuario = $1
          AND fechahora > $2
          AND id_recorrido IS NOT NULL
          AND operacion IS NOT NULL
          AND fechahora IS NOT NULL
        ORDER BY fechahora ASC, id_recorrido ASC
        LIMIT 1
        "#,
    )
    .bind(id_usuario as i64)
    .bind(fechahora)
    .fetch_optional(executor)
    .await
    .context("no se pudo consultar siguiente movimiento despues de fecha")?;

    row.map(movimiento_desde_fila).transpose()
}

async fn insertar_movimiento_executor<'e, E>(
    executor: E,
    movimiento: &MovimientoRecorrido,
) -> anyhow::Result<()>
where
    E: Executor<'e, Database = Postgres>,
{
    let operacion = operacion_texto(movimiento.operacion);
    let id_recorrido = movimiento.id_recorrido as i64;
    let id_usuario = movimiento.id_usuario as i64;
    let id_estacion = movimiento.id_estacion.map(|id| id as i64);
    let estacion = id_estacion
        .map(|id| id.to_string())
        .unwrap_or_else(|| "sin-estacion".to_string());
    let finalizado_en =
        (movimiento.operacion == Operacion::Devolucion).then_some(movimiento.fechahora);

    sqlx::query(
        r#"
        INSERT INTO recorridos (
            bicicleta_id,
            estacion_origen_id,
            estacion_destino_id,
            iniciado_en,
            finalizado_en,
            id_recorrido,
            id_usuario,
            operacion,
            fechahora,
            id_estacion
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $4, $9)
        "#,
    )
    .bind(id_recorrido.to_string())
    .bind(estacion)
    .bind(Option::<String>::None)
    .bind(movimiento.fechahora)
    .bind(finalizado_en)
    .bind(id_recorrido)
    .bind(id_usuario)
    .bind(operacion)
    .bind(id_estacion)
    .execute(executor)
    .await
    .context("no se pudo registrar movimiento en recorridos")?;

    Ok(())
}

fn movimiento_desde_fila(row: PgRow) -> anyhow::Result<MovimientoRecorrido> {
    let operacion: String = row.try_get("operacion")?;
    let id_estacion: Option<i64> = row.try_get("id_estacion")?;

    Ok(MovimientoRecorrido {
        id_recorrido: row.try_get::<i64, _>("id_recorrido")? as u64,
        id_usuario: row.try_get::<i64, _>("id_usuario")? as u64,
        id_estacion: id_estacion.map(|id| id as u64),
        operacion: match operacion.as_str() {
            "retiro" => Operacion::Retiro,
            "devolucion" => Operacion::Devolucion,
            otro => anyhow::bail!("operacion invalida en recorridos: {otro}"),
        },
        fechahora: row.try_get::<DateTime<Utc>, _>("fechahora")?,
    })
}

fn operacion_texto(operacion: Operacion) -> &'static str {
    match operacion {
        Operacion::Retiro => "retiro",
        Operacion::Devolucion => "devolucion",
    }
}
