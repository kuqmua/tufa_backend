use std::time::Duration;

use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;

pub async fn do_smth() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new("url".to_owned());
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    Ok(db)
}