// app_state_mod.rs

use std::collections::HashMap;

use crate::postgres_mod::{FieldName, FunctionName, ParamName, ViewName};
use crate::postgres_type_mod::{PostgresFieldType, PostgresInputType};

/// This struct represents state
/// Every function can extract this simply with an input parameter
/// fun (app_state: actix_web::web::Data<AppState>)
pub struct AppState {
    pub app_name: String,
    pub db_pool: deadpool_postgres::Pool,
    pub sql_function_input_params: HashMap<FunctionName, HashMap<ParamName, PostgresInputType>>,
    pub sql_view_fields: HashMap<ViewName, HashMap<FieldName, PostgresFieldType>>,
}
