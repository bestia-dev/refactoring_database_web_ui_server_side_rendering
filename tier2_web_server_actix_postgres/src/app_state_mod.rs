// app_state_mod.rs

use crate::postgres_mod::{SqlFunctionInputParams, SqlFunctionInputParamsOrder, SqlViewFields};

/// This struct represents state
/// Every function can extract this simply with an input parameter
/// fun (app_state: actix_web::web::Data<AppState>)
pub struct AppState {
    pub app_name: String,
    pub db_pool: deadpool_postgres::Pool,
    pub sql_function_input_params: SqlFunctionInputParams,
    pub sql_function_input_params_order: SqlFunctionInputParamsOrder,
    pub sql_view_fields: SqlViewFields,
}
