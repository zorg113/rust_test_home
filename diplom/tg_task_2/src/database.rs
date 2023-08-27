use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::migration::DbErr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database as SeaOrmDatabase, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};

#[derive(Debug)]
pub enum Error {
    Database(DbErr),
    File(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Database(ref err) => {
                write!(f, "Database error: {}", err)
            }
            Self::File(ref err) => write!(f, "File error: {}", err),
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Self::Database(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::File(err)
    }
}

async fn get_db_pool(db_path: &PathBuf) -> Result<DatabaseConnection, Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_path)?;
    let db_str = format!("sqlite:{}", db_path.display());
    let pool = SeaOrmDatabase::connect(&db_str).await.unwrap();
    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub async fn new(db_path: &PathBuf) -> Result<Self, Error> {
        get_db_pool(db_path).await.map(|pool| Self { pool })
    }

    pub async fn apply_migrations(&self) -> Result<(), Error> {
        Ok(())
    }
}
