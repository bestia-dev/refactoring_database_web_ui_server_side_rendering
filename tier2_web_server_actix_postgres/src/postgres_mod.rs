//! postgres_mod.rs

use crate::error_mod::LibError;
use tokio_postgres::error::SqlState;
/// return the single row
/// check if there is less or more rows and return error
/// the row_set is consumed
#[track_caller]
pub fn get_single_row_owned(
    row_set: Vec<tokio_postgres::Row>,
) -> Result<tokio_postgres::Row, LibError> {
    if row_set.len() != 1 {
        let source_caller_location = std::panic::Location::caller();
        return Err(LibError::NotSingleRow {
            user_friendly: "".to_string(),
            developer_friendly: "".to_string(),
            source_line_column: format!(
                "{}:{}:{}",
                source_caller_location.file(),
                source_caller_location.line(),
                source_caller_location.column()
            ),
        });
    }
    let mut row_set = row_set;
    // take ownership of the Row. The row_set will become empty!
    let single_row = row_set.remove(0);
    Ok(single_row)
}
type DbPool = actix_web::web::Data<deadpool_postgres::Pool>;
/// run the query and catch the many different sql errors
#[track_caller]
pub async fn run_sql_select_query(
    db_pool: DbPool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<tokio_postgres::Row>, LibError> {
    let client = crate::deadpool_mod::get_client_from_pool(db_pool).await?;
    client.query(query, params).await.map_err(|err| {
        /*
        many different sql errors:
        https://github.com/sfackler/rust-postgres/blob/master/tokio-postgres/src/error/sqlstate.rs
        code: SqlState( E42804, ), DATATYPE_MISMATCH
            message: "structure of query does not match function result type",
            detail: Some( "Returned type character varying(100) does not match expected type text in column 2.", ),
            where_: Some( "PL/pgSQL function webpage_hits_insert(integer,text,integer) line 12 at RETURN QUERY", ),
        code: SqlState( E23505, ), SqlState::UNIQUE_VIOLATION
            message: "duplicate key value violates unique constraint \"webpage_uniq_webpage\"",
            detail: Some( "Key (webpage)=(test) already exists.", ),
            where_: Some( "SQL statement \"insert into webpage ( \"id\", webpage)\nvalues (_id, _webpage)\"\nPL/pgSQL function webpage_hits_insert(integer,text,integer) line 6 at SQL statement", ),
        */
        let err_code = err.code().unwrap().clone();
        match err_code {
            SqlState::UNIQUE_VIOLATION =>
            // duplicate key value violates unique constraint
            {
                LibError::QueryError {
                    user_friendly: format!("{}", err),
                    source_error: err,
                    developer_friendly: format!("{:?} {} {:?}", err_code, query, params),
                    source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
                }
            }
            SqlState::DATATYPE_MISMATCH =>
            // structure of query does not match function result type
            {
                LibError::QueryError {
                    user_friendly: format!("{}", err),
                    source_error: err,
                    developer_friendly: format!("{:?} {} {:?}", err_code, query, params),
                    source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
                }
            }
            _ => LibError::QueryError {
                user_friendly: format!("{}", err),
                source_error: err,
                developer_friendly: format!("{} {:?}", query, params),
                source_line_column: format!("{}:{}:{}", file!(), line!(), column!()),
            },
        }
    })
}
#[track_caller]
pub async fn run_sql_void_function(
    db_pool: DbPool,
    function_name: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<(), LibError> {
    // TODO: from params deduce parameters
    let placeholders = "$1";
    let query = format!("SELECT {function_name}({placeholders});");
    let _row_set = run_sql_select_query(db_pool, &query, params).await?;
    Ok(())
}
#[track_caller]
pub async fn run_sql_single_row_function(
    db_pool: DbPool,
    function_name: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<tokio_postgres::Row, LibError> {
    let placeholders = prepare_placeholders_for_params(params);
    let query = format!("SELECT * from {function_name}({placeholders});");
    let row_set = run_sql_select_query(db_pool, &query, params).await?;
    let single_row = get_single_row_owned(row_set)?;
    Ok(single_row)
}
/// from params deduce parameters for placeholders in sql queries
fn prepare_placeholders_for_params(
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> String {
    let mut placeholders = String::new();
    let mut delimiter = String::new();
    // starts from 1
    for i in 1..=params.len() {
        placeholders.push_str(&format!("{delimiter}${i}"));
        if delimiter.is_empty() {
            delimiter.push_str(", ");
        }
    }
    placeholders
}
