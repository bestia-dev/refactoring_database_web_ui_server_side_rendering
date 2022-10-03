// refactoring_database_web_ui_server_side_rendering/tier2_web_server_actix_postgres/src/lib.rs

mod actix_mod;
mod app_state_mod;
mod deadpool_mod;
mod error_mod;
mod html_templating_mod;
mod postgres_mod;
mod postgres_type_mod;
mod server_side_multi_row_mod;
mod server_side_single_row_mod;
mod webpage_hits_mod;

pub use actix_mod::config_route_main;
pub use app_state_mod::AppState;
pub use deadpool_mod::deadpool_start_and_check;
pub use postgres_mod::get_for_cache_all_function_input_params;
pub use postgres_mod::get_for_cache_all_view_fields;
