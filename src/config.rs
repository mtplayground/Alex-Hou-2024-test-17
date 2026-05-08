use std::{env, io};

use tracing_subscriber::{fmt, EnvFilter};

pub struct AppEnv {
    database_url: String,
    rust_log: String,
}

impl AppEnv {
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        dotenvy::dotenv().ok();

        let database_url = read_required_env("DATABASE_URL")?;
        let rust_log = read_required_env("RUST_LOG")?;
        let site_addr = read_required_env("LEPTOS_SITE_ADDR")?;

        env::set_var("LEPTOS_SITE_ADDR", &site_addr);

        Ok(Self {
            database_url,
            rust_log,
        })
    }

    pub fn init_tracing(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let env_filter = EnvFilter::try_new(self.rust_log.as_str())?;

        fmt()
            .with_env_filter(env_filter)
            .with_target(false)
            .compact()
            .try_init()?;

        Ok(())
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }
}

fn read_required_env(key: &str) -> Result<String, io::Error> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{key} must not be empty"),
        )),
        Err(env::VarError::NotPresent) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{key} is required"),
        )),
        Err(env::VarError::NotUnicode(_)) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{key} must be valid unicode"),
        )),
    }
}
