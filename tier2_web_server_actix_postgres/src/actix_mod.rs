// refactoring_database_web_ui_server_side_rendering/tier2_web_server_actix_postgres/src/actix_mod.rs

use crate::error_mod::LibError;

/// configure the route with scope
/// so the routing code is near to the implementation code
pub fn config_route_main(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_files::Files::new(
        "/webpage_hits_admin/css",
        "./webpage_hits_admin/css/",
    ))
    .service(
        actix_web::web::scope("/webpage_hits_admin/webpage_hits")
            .configure(crate::webpage_hits_mod::config_route_webpage_hits),
    );
}

/// fn to return a response when we have the body
pub fn return_response_no_cache(body: String) -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
        .append_header(actix_web::http::header::ContentType(mime::TEXT_HTML_UTF_8))
        .append_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::NoStore,
        ]))
        .body(body))
}

/// data from GET query as &str
/// [("id", "496953237"), ("webpage", "webpage short url"), ("hit_count", "0")]
/// track_caller makes std::panic::Location::caller() return the caller location
#[track_caller]
pub fn get_str_from_web_query<'a>(
    query: &'a actix_web::web::Query<Vec<(String, String)>>,
    name: &str,
) -> Result<&'a str, LibError> {
    for x in query.0.iter() {
        if x.0 == name {
            return Ok(&x.1);
        }
    }
    let source_caller_location = std::panic::Location::caller();
    Err(LibError::GetValueFromWebQuery {
        user_friendly: name.to_string(),
        developer_friendly: format!("{:?}", query),
        source_line_column: format!(
            "{}:{}:{}",
            source_caller_location.file(),
            source_caller_location.line(),
            source_caller_location.column()
        ),
    })
}
/// data from GET query as i32
/// [("id", "496953237"), ("webpage", "webpage short url"), ("hit_count", "0")]
/// track_caller makes std::panic::Location::caller() return the caller location
#[track_caller]
pub fn get_i32_from_web_query<'a>(
    query: &'a actix_web::web::Query<Vec<(String, String)>>,
    name: &str,
) -> Result<i32, LibError> {
    let value_str = get_str_from_web_query(&query, name)?;
    let source_caller_location = std::panic::Location::caller();
    let value_i32: i32 = value_str
        .parse::<i32>()
        .map_err(|_err| LibError::GetI32FromWebQuery {
            user_friendly: value_str.to_string(),
            developer_friendly: format!("{:?}", query),
            source_line_column: format!(
                "{}:{}:{}",
                source_caller_location.file(),
                source_caller_location.line(),
                source_caller_location.column()
            ),
        })?;
    Ok(value_i32)
}
