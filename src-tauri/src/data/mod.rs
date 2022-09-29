use sea_orm::{DbErr, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

mod migrator;

use migrator::Migrator;

pub async fn connect(
    db_url: &str,
) -> Result<DatabaseConnection, DbErr> {
    match Database::connect(db_url).await {
        Ok(c) => Ok(c),
        Err(e) => Err(e),
    }
}

pub async fn init(
    db_url: &str,
) -> Result<(), DbErr> {
    match Database::connect(db_url).await {
        Ok(c) => {
            match Migrator::refresh(&c).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}
