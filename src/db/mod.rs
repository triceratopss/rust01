use crate::AppConfig;

use sea_orm::*;

pub(super) async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name,
    ));

    opts.sqlx_logging(false);

    Database::connect(opts).await
}
