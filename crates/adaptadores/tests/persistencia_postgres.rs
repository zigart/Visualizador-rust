use std::path::PathBuf;

use adaptadores::postgres::{estado, recorrido, ErrorPersistencia, PersistenciaMovimientos};
use chrono::{TimeZone, Utc};
use dominio::{ErrorDominio, MovimientoRecorrido, Operacion};
use sqlx::PgPool;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

async fn pool_con_migraciones() -> (testcontainers::ContainerAsync<Postgres>, PgPool) {
    let container = Postgres::default()
        .start()
        .await
        .expect("no se pudo iniciar PostgreSQL de testcontainers");

    let host = container.get_host().await.expect("host postgres");
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("puerto postgres");
    let url = format!("postgres://postgres:postgres@{host}:{port}/postgres");

    let pool = PgPool::connect(&url)
        .await
        .expect("no se pudo conectar a PostgreSQL de test");

    let migrations = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    sqlx::migrate::Migrator::new(migrations)
        .await
        .expect("migrator invalido")
        .run(&pool)
        .await
        .expect("no se pudieron aplicar migraciones");

    (container, pool)
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
        id_estacion: Some(100 + id_recorrido),
        operacion,
        fechahora: Utc.with_ymd_and_hms(2026, 7, dia, 12, 0, 0).unwrap(),
    }
}

#[tokio::test]
async fn inserta_retiro_y_actualiza_estado() {
    let (_container, pool) = pool_con_migraciones().await;
    let persistencia = PersistenciaMovimientos::new(pool.clone());

    let estado = persistencia
        .persistir_movimiento(&movimiento(1, 10, Operacion::Retiro, 1))
        .await
        .expect("retiro valido");

    assert_eq!(estado.en_uso, 1);
    assert_eq!(estado.maximo_historico, 1);

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM recorridos WHERE id_recorrido = 1 AND operacion = 'retiro'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 1);

    let en_uso: i32 = sqlx::query_scalar("SELECT en_uso FROM estado_bicicletas WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(en_uso, 1);
}

#[tokio::test]
async fn devolucion_decrementa_en_uso() {
    let (_container, pool) = pool_con_migraciones().await;
    let persistencia = PersistenciaMovimientos::new(pool.clone());

    persistencia
        .persistir_movimiento(&movimiento(1, 10, Operacion::Retiro, 1))
        .await
        .unwrap();
    persistencia
        .persistir_movimiento(&movimiento(2, 20, Operacion::Retiro, 1))
        .await
        .unwrap();
    persistencia
        .persistir_movimiento(&movimiento(3, 30, Operacion::Retiro, 1))
        .await
        .unwrap();

    estado::actualizar_estado_pool(&pool, dominio::EstadoBicicletas::new(3, 5))
        .await
        .unwrap();

    let estado = persistencia
        .persistir_movimiento(&movimiento(2, 20, Operacion::Devolucion, 3))
        .await
        .expect("devolucion valida");

    assert_eq!(estado.en_uso, 2);
    assert_eq!(estado.maximo_historico, 5);
}

#[tokio::test]
async fn consulta_ultimo_y_siguiente_movimiento() {
    let (_container, pool) = pool_con_migraciones().await;
    let persistencia = PersistenciaMovimientos::new(pool.clone());

    persistencia
        .persistir_movimiento(&movimiento(1, 10, Operacion::Retiro, 1))
        .await
        .unwrap();
    persistencia
        .persistir_movimiento(&movimiento(1, 10, Operacion::Devolucion, 3))
        .await
        .unwrap();
    persistencia
        .persistir_movimiento(&movimiento(2, 10, Operacion::Retiro, 5))
        .await
        .unwrap();

    let fecha = Utc.with_ymd_and_hms(2026, 7, 3, 12, 0, 0).unwrap();
    let ultimo = recorrido::ultimo_movimiento_antes(&pool, 10, fecha)
        .await
        .unwrap()
        .expect("ultimo movimiento");
    assert_eq!(ultimo.operacion, Operacion::Devolucion);
    assert_eq!(ultimo.id_recorrido, 1);

    let fecha = Utc.with_ymd_and_hms(2026, 7, 1, 11, 0, 0).unwrap();
    let siguiente = recorrido::siguiente_movimiento_despues(&pool, 10, fecha)
        .await
        .unwrap()
        .expect("siguiente movimiento");
    assert_eq!(siguiente.operacion, Operacion::Retiro);
    assert_eq!(siguiente.id_recorrido, 1);
}

#[tokio::test]
async fn rechaza_retiro_duplicado_sin_insertar() {
    let (_container, pool) = pool_con_migraciones().await;
    let persistencia = PersistenciaMovimientos::new(pool.clone());

    persistencia
        .persistir_movimiento(&movimiento(1, 10, Operacion::Retiro, 1))
        .await
        .unwrap();

    let error = persistencia
        .persistir_movimiento(&movimiento(1, 20, Operacion::Retiro, 2))
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        ErrorPersistencia::Dominio(ErrorDominio::IdRecorridoYaUtilizado { id_recorrido: 1 })
    ));

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM recorridos")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn esquema_alineado_sin_columnas_legacy() {
    let (_container, pool) = pool_con_migraciones().await;

    let columnas_legacy: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT column_name::TEXT
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'recorridos'
          AND column_name IN (
              'bicicleta_id',
              'estacion_origen_id',
              'estacion_destino_id',
              'iniciado_en',
              'finalizado_en',
              'created_at',
              'updated_at'
          )
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(columnas_legacy.is_empty());

    let updated_at: Option<String> = sqlx::query_scalar(
        r#"
        SELECT column_name::TEXT
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'estado_bicicletas'
          AND column_name = 'updated_at'
        "#,
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    assert!(updated_at.is_none());

    let columnas_recorridos: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT column_name::TEXT
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'recorridos'
        ORDER BY ordinal_position
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(
        columnas_recorridos,
        vec![
            "id".to_string(),
            "id_recorrido".to_string(),
            "id_usuario".to_string(),
            "operacion".to_string(),
            "fechahora".to_string(),
            "id_estacion".to_string(),
        ]
    );
}

#[tokio::test]
async fn rollback_no_deja_movimiento_si_falla_actualizacion_estado() {
    let (_container, pool) = pool_con_migraciones().await;
    let movimiento = movimiento(1, 10, Operacion::Retiro, 1);

    let mut tx = pool.begin().await.unwrap();
    let conn: &mut sqlx::PgConnection = &mut *tx;
    recorrido::insertar_movimiento(conn, &movimiento)
        .await
        .unwrap();

    let resultado = sqlx::query(
        "INSERT INTO recorridos (id_recorrido, id_usuario, operacion, fechahora) VALUES (99, 99, 'retiro', NULL)",
    )
    .execute(conn)
    .await;
    assert!(resultado.is_err());
    tx.rollback().await.unwrap();

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM recorridos")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
