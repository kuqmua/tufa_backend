use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;

pub async fn do_smth() -> Result<DatabaseConnection, DbErr> {
    let db: DatabaseConnection = Database::connect("url").await?;
    Ok(db)
}