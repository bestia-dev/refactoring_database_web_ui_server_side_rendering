// refactoring_database_web_ui_server_side_rendering/tier2_web_server_actix_postgres/src/bin/webpage_hits_admin/main.rs
use tier2_web_server_actix_postgres as tier2;

/// the binary executable entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    println!("Actix web server started on localhost:8080!");
    println!("Test it with curl or browser:");
    println!("http://localhost:8080/webpage_hits_admin/webpage_hits/webpage_hits_list");

    // connection pool for postgres to reuse connections for better performance
    let db_pool = tier2::deadpool_start_and_check().await;

    // on start get all the input parameters for sql functions.
    // So I can parse string params to a correct rust data type.
    let (sql_function_input_params, sql_function_input_params_order) =
        tier2::get_for_cache_all_function_input_params(&db_pool).await;

    // I need the view fields and types to construct the WHERE clause
    let sql_view_fields = tier2::get_for_cache_all_view_fields(&db_pool).await;

    // Create web::Data outside of closure HttpServer::new.
    let app_state = actix_web::web::Data::new(tier2::AppState {
        app_name: String::from("bestia.dev"),
        db_pool: db_pool,
        sql_function_input_params,
        sql_function_input_params_order,
        sql_view_fields,
    });

    let http_server_result = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            // app_data is cloned for every worker thread
            .app_data(app_state.clone())
            // the route is configured near the implementation code
            .configure(tier2::config_route_main)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    println!("");
    println!("Actix web server stopped!");
    // return
    http_server_result
}
