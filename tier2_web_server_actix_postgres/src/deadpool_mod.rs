//! deadpool_mod.rs

type DbPool = actix_web::web::Data<deadpool_postgres::Pool>;
use crate::error_mod::LibError;

/// create and start the connection pool
pub async fn deadpool_postgres_start() -> deadpool_postgres::Pool {
    // this loads our .env file and includes the values in std::env
    println!("Reading dotenv");
    dotenv::dotenv().ok();
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(std::env::var("PG.HOST").unwrap().as_str());
    pg_config.user(std::env::var("PG.USER").unwrap().as_str());
    pg_config.dbname(std::env::var("PG.DBNAME").unwrap().as_str());
    let mgr_config = deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    };
    let mgr = deadpool_postgres::Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
    println!("Create pool");
    let pool = deadpool_postgres::Pool::builder(mgr)
        .max_size(16)
        .build()
        .unwrap();
    // return
    pool
}

/// start and check the connection pool to postgres
pub async fn deadpool_start_and_check() -> deadpool_postgres::Pool {
    let pool = crate::deadpool_mod::deadpool_postgres_start().await;
    // Check the connection to postgres database and panic if error
    let _client: deadpool_postgres::Client = pool.get().await.unwrap();
    pool
}

/// get client from pool
pub async fn get_client_from_pool(db_pool: DbPool) -> Result<deadpool_postgres::Object, LibError> {
    db_pool
        .get()
        .await
        .map_err(|_| crate::error_mod::LibError::DatabaseConnection)
}
