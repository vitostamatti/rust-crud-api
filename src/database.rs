use crate::configuration::DatabaseConfiguration;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Error, PgPool};

pub async fn get_db(config: &DatabaseConfiguration) -> Result<PgPool, Error> {
    let connection_options = PgConnectOptions::new()
        .host(&config.host)
        .username(&config.username)
        .password(&config.password)
        .port(config.port)
        .database(&config.database_name);

    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with(connection_options)
        .await
}
