use sqlx::error::Error as SqlxErr;
use sqlx::postgres::PgDatabaseError;
use std::error::Error;
use std::fmt;

// PG constraints:
// URL: https://www.postgresql.org/docs/current/errcodes-appendix.html#ERRCODES-TABLE
//
//     Class 23 — Integrity Constraint Violation
//         23000    Integrity constraint violation
//         23001    Restrict violation
//         23502    Not null violation
//         23503    Foreign key violation
//         23505    Unique violation
//         23514    Check violation
//         23P01    Exclusion violation
//
//      Class 22 — Data Exception
//         22000    data_exception
//
//      Class 22 — Data Exception
//         22000    data_exception
//
//      Class P0 — PL/pgSQL Error
//         P0001    raise_exception
//
//      Class P0 — PL/pgSQL Error
//         P0000    plpgsql_error
//         P0002    no_data_found
//         P0003    too_many_rows
//         P0004    assert_failure

#[derive(Debug)]
pub struct PgError {
    pub code: String,
    column: Option<String>,
    pub constraint: Option<String>,
    // violation_type: std::cell::Cell<Option<String>>,
    pub violation_type: Option<String>,
    data_type: Option<String>,
    detail: Option<String>,
    hint: Option<String>,
    pub message: String,
    routine: Option<String>,
    table: Option<String>,
    schema: Option<String>,
}

impl From<&PgDatabaseError> for PgError {
    fn from(v: &PgDatabaseError) -> Self {
        let p = v.column().to_owned();
        let p = v.message();
        Self {
            code: v.code().to_owned(),
            column: v.column().map(|v| v.to_owned()),
            constraint: v.constraint().map(|v| v.to_owned()),
            data_type: v.data_type().map(|v| v.to_owned()),
            detail: v.detail().map(|v| v.to_owned()),
            hint: v.hint().map(|v| v.to_owned()),
            message: v.message().to_owned(),
            routine: v.routine().map(|v| v.to_owned()),
            table: v.table().map(|v| v.to_owned()),
            schema: v.schema().map(|v| v.to_owned()),
            violation_type: None,
        }
    }
}

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Constraint(PgError),
}

impl Error for DbError {}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound => todo!(),
            DbError::Constraint(_) => todo!(),
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        log::error!("{error:?}");
        match &error {
            SqlxErr::Configuration(_) => todo!(),
            SqlxErr::Database(e) => {
                let mut err: PgError = PgError::from(e.downcast_ref::<PgDatabaseError>());

                match err.code.as_str() {
                    "23000" => {
                        err.violation_type = Some("Integrity constraint violation".to_owned())
                    }
                    "23001" => err.violation_type = Some("Restrict violation".to_owned()),
                    "23502" => err.violation_type = Some("Not null violation".to_owned()),
                    "23503" => err.violation_type = Some("Foreign key violation".to_owned()),
                    "23505" => err.violation_type = Some("Unique violation".to_owned()),
                    "23514" => err.violation_type = Some("Check violation".to_owned()),
                    "23P01" => err.violation_type = Some("Exclusion violation".to_owned()),
                    _ => todo!(),
                };
                DbError::Constraint(err)
            }
            SqlxErr::RowNotFound => DbError::NotFound,
            _ => todo!(),
        }
    }
}
