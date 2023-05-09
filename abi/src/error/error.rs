use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("Database error")]
    DbError(sqlx::Error),
    #[error("Invalid userId: {0}")]
    InvalidUserId(&'a String),
    #[error("Invalid resourceId: {0}")]
    InvalidResourceId(&'a String),
    #[error("conflict error")]
    ConflictError(),
    #[error("has not found")]
    NotFound,
    #[error("unknown error")]
    Unknown,
}

impl From<sqlx::Error> for Error<'_> {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("reservations")) => Error::ConflictError(),
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}
