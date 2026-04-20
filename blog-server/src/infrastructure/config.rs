use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub grpc_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()?;
        let grpc_port = std::env::var("GRPC_PORT")
            .unwrap_or_else(|_| "50051".into())
            .parse()?;
        let jwt_secret = std::env::var("JWT_SECRET")?;

        Ok(Self {
            database_url,
            host,
            port,
            grpc_port,
            jwt_secret,
        })
    }
}
