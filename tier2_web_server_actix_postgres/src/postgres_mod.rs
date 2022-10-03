//! postgres_mod.rs

// newtypes : forces unambiguous intent
#[derive(Eq, Hash, PartialEq)]
pub struct FunctionName(pub String);
#[derive(Eq, Hash, PartialEq)]
pub struct ParamName(pub String);
#[derive(Eq, Hash, PartialEq)]
pub struct ViewName(pub String);
#[derive(Eq, Hash, PartialEq)]
pub struct FieldName(pub String);

use crate::{
    error_mod::LibError,
    postgres_type_mod::{PostgresFieldType, PostgresInputType},
};
use std::collections::HashMap;
use tokio_postgres::error::SqlState;

/// run the query and catch the many different sql errors
#[track_caller]
pub async fn run_sql_select_query_pool(
    db_pool: &deadpool_postgres::Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<tokio_postgres::Row>, LibError> {
    let postgres_client = crate::deadpool_mod::get_postgres_client_from_pool(db_pool).await?;
    postgres_client.query(query, params).await.map_err(|err| {
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

/// from params deduce parameters for placeholders in sql queries
pub fn prepare_placeholders_for_sql_params(
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

/// Vector of all function input params with data types.
/// Call it once on application start and store the result in a global variable.
/// Postgres input variables can be prefixed with "in_" or just "_". Take it into consideration.
pub async fn get_for_cache_all_function_input_params(
    db_pool: &deadpool_postgres::Pool,
) -> HashMap<FunctionName, HashMap<ParamName, PostgresInputType>> {
    let query = "SELECT proname, args_def from get_function_input_params;";
    let vec_row = run_sql_select_query_pool(db_pool, query, &[])
        .await
        .unwrap();
    let mut function_input_params: HashMap<FunctionName, HashMap<ParamName, PostgresInputType>> =
        HashMap::new();
    for row in vec_row.iter() {
        // newtype
        let function_name = FunctionName(row.get(0));
        //dbg!(&function_name);
        let args_def: String = row.get(1);
        //dbg!(&args_def);
        let mut hm_name_type: HashMap<ParamName, PostgresInputType> = HashMap::new();
        if !args_def.is_empty() {
            //parse into Vec<(String,String)>
            for name_and_type in args_def.split(", ") {
                let mut spl = name_and_type.split(' ');
                let param_name = ParamName(spl.next().unwrap().to_string());
                // ignore OUT parameters, only input parameters
                if param_name.0 != "OUT" {
                    let arg_type = spl.next().unwrap().to_string();
                    use std::str::FromStr;
                    let arg_type = PostgresInputType::from_str(&arg_type).unwrap();
                    hm_name_type.insert(param_name, arg_type);
                }
            }
        }
        //dbg!(&vec_name_type);
        function_input_params.insert(function_name, hm_name_type);
    }
    function_input_params
}

/// Hashmap of all view fields with data types. I use it to construct the WHERE clause.
/// Call it once on application start and store the result in a global variable.
pub async fn get_for_cache_all_view_fields(
    db_pool: &deadpool_postgres::Pool,
) -> HashMap<ViewName, HashMap<FieldName, PostgresFieldType>> {
    let query = "SELECT relname, attname, typname from get_view_fields order by relname;";
    let vec_row = run_sql_select_query_pool(db_pool, query, &[])
        .await
        .unwrap();

    let mut view_fields: HashMap<ViewName, HashMap<FieldName, PostgresFieldType>> = HashMap::new();
    let mut hm_name_type = HashMap::new();

    let mut old_relname = ViewName(String::new());
    let mut relname: ViewName;
    for row in vec_row.iter() {
        relname = ViewName(row.get(0));
        if relname != old_relname {
            if !old_relname.0.is_empty() {
                //dbg!(&vec_name_type);
                view_fields.insert(old_relname, hm_name_type);
                hm_name_type = HashMap::new();
            }
            old_relname = relname;
        }
        //dbg!(&relname);
        let attname = FieldName(row.get(1));
        //dbg!(&attname);
        let typname: String = row.get(2);
        //dbg!(&typname);
        use std::str::FromStr;
        let arg_type = PostgresFieldType::from_str(&typname).unwrap();
        hm_name_type.insert(attname, arg_type);
    }
    if !old_relname.0.is_empty() {
        //dbg!(&vec_name_type);
        view_fields.insert(old_relname, hm_name_type);
    }
    // dbg!(&view_fields);
    view_fields
}
