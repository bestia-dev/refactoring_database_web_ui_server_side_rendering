//! webpage_hits_mod.rs

// type aliases: for less verbose types and better readability of the code

use crate::actix_mod::{DataAppState, ResultResponse, WebForm, WebQuery};
use crate::server_side_multi_row_mod::ServerSideMultiRow;
use crate::server_side_single_row_mod::ServerSideSingleRow;
use actix_web::web::resource;
use actix_web::web::to;

const SCOPE: &'static str = "webpage_hits";

/// scoped actix routing near the implementation code
/// scope is already "/webpage_hits_admin/webpage_hits"
pub fn config_route_webpage_hits(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(resource("/webpage_hits_list").route(to(webpage_hits_list)))
        .service(resource("/webpage_hits_new").route(to(webpage_hits_new)))
        .service(resource("/webpage_hits_edit").route(to(webpage_hits_edit)))
        .service(resource("/webpage_hits_insert").route(to(webpage_hits_insert)))
        .service(resource("/webpage_hits_show").route(to(webpage_hits_show)))
        .service(resource("/webpage_hits_update").route(to(webpage_hits_update)))
        .service(resource("/webpage_hits_delete").route(to(webpage_hits_delete)));
}

/// CRUD - read (list all webpages and counts) with simple filter and order_by
#[function_name::named]
pub async fn webpage_hits_list(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut ssmr = ServerSideMultiRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    // The where statement is constructed only for existing parameters, because efficiency.
    ssmr.where_clause = vec![
        "webpage like {f_like_webpage}",
        "hit_counter > {f_gt_hit_counter}",
        "hit_counter < {f_lt_hit_counter}",
    ];

    ssmr.run_multi_row_sql_and_process_html().await
}

/// UI - new record
#[function_name::named]
pub async fn webpage_hits_new(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    sssr.run_single_row_sql_and_process_html().await
}

/// UI - edit record
#[function_name::named]
pub async fn webpage_hits_edit(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    sssr.run_single_row_sql_and_process_html().await
}

/// CRUD - create(insert)
#[function_name::named]
pub async fn webpage_hits_insert(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    sssr.run_single_row_sql_and_process_html().await
}

/// CRUD - read (show one record)
#[function_name::named]
pub async fn webpage_hits_show(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    sssr.run_single_row_sql_and_process_html().await
}

/// CRUD - update
#[function_name::named]
pub async fn webpage_hits_update(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    sssr.run_single_row_sql_and_process_html().await
}

/// CRUD - delete
#[function_name::named]
pub async fn webpage_hits_delete(
    app_state: DataAppState,
    query: WebQuery,
    form: Option<WebForm>,
) -> ResultResponse {
    let mut sssr = ServerSideSingleRow::new(&app_state, SCOPE, function_name!(), &query, &form);
    sssr.run_single_row_sql_and_process_html().await
}
