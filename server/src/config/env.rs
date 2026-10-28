
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub app_env: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub log_format: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        Ok(Self {
            app_name: required("APP_NAME")?,
            app_env: required("APP_ENV")?,
            host: required("HOST")?,
            port: required("PORT")?
                .parse()
                .context("PORT must be a valid number")?,
            log_level: required("LOG_LEVEL")?,
            log_format: required("LOG_FORMAT")?,
            database_url: required("DATABASE_URL")?,
        })
    }
}

fn required(key: &str) -> Result<String> {
    let value = std::env::var(key)
        .with_context(|| format!("Missing environment variable: {key}"))?;

    if value.trim().is_empty() {
        anyhow::bail!("Environment variable {key} cannot be empty");
    }

    Ok(value)
}
