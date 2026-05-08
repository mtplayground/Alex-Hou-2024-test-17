use std::{path::Path, str::FromStr};

use sqlx::{
    migrate::Migrator,
    sqlite::SqlitePoolOptions,
    Pool,
    Sqlite,
    sqlite::SqliteConnectOptions,
};

pub type DbPool = Pool<Sqlite>;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn connect(database_url: &str) -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    ensure_parent_directory(options.get_filename())?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    tracing::info!("connected to SQLite database");

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    MIGRATOR.run(pool).await?;
    tracing::info!("database migrations completed");
    Ok(())
}

fn ensure_parent_directory(path: &Path) -> Result<(), std::io::Error> {
    if path == Path::new(":memory:") {
        return Ok(());
    }

    if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }

    Ok(())
}
