// refactoring_database_web_ui_server_side_rendering/tier2_web_server_actix_postgres/src/actix_mod.rs

// type aliases: for less verbose types and better readability of the code
pub type WebForm = actix_web::web::Form<Vec<(String, String)>>;
pub type WebQuery = actix_web::web::Query<Vec<(String, String)>>;
pub type ResultResponse = actix_web::Result<actix_web::HttpResponse>;
pub type DataAppState = actix_web::web::Data<crate::AppState>;

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
/// web apps modify data all the time, so caching is not good
pub fn return_response_no_cache(body: String) -> actix_web::Result<actix_web::HttpResponse> {
    use actix_web::http::header;
    Ok(actix_web::HttpResponse::Ok()
        .append_header(header::ContentType(mime::TEXT_HTML_UTF_8))
        .append_header(header::CacheControl(vec![header::CacheDirective::NoStore]))
        .body(body))
}
