// refactoring_database_web_ui_server_side_rendering/tier2_web_server_actix_postgres/src/actix_mod.rs

use std::collections::HashMap;

use crate::error_mod::{file_line_column, LibError};

type WebForm = actix_web::web::Form<Vec<(String, String)>>;
type WebQuery = actix_web::web::Query<Vec<(String, String)>>;

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

/// WebParams are just a key-value collection: HashMap(String,String)  
/// A similar collection is found inside POST(form) and GET(web query)  
/// But I want a collection independent of the POST/GET method  
/// [("id", "496953237"), ("webpage", "webpage short url"), ("hit_count", "0")]  
#[derive(Debug)]
pub struct WebParams(pub HashMap<String, String>);

impl WebParams {
    /// get WebParams from POST(form) if exists or else GET(web query)  
    /// If Post(form) exists, then GET(web query) is ignored.  
    /// track_caller decoration makes Location::caller() return the caller location  
    /// for meaningful source code location of the actual error  
    #[track_caller]
    pub fn from_actix(query: &WebQuery, form: &Option<WebForm>) -> WebParams {
        if let Some(form) = form {
            // into_iter() consumes the vector. The vector cannot be used after calling this.
            WebParams(form.0.clone().into_iter().collect())
        } else {
            WebParams(query.0.clone().into_iter().collect())
        }
    }

    /// data from WebParams as &str  
    #[track_caller]
    pub fn get_str(&self, param_name: &str) -> Result<&str, LibError> {
        // convert from Option-None to Error with .ok_or()
        let value = self
            .0
            .get(param_name)
            .ok_or(LibError::GetStrFromWebParams {
                user_friendly: param_name.to_string(),
                developer_friendly: format!("{:?}", self.0),
                source_line_column: file_line_column(&std::panic::Location::caller()),
            })?;

        Ok(value)
    }

    /// data from WebParams as i32  
    #[track_caller]
    pub fn get_i32(&self, param_name: &str) -> Result<i32, LibError> {
        let value = self.get_str(param_name)?.parse::<i32>().map_err(|_err| {
            LibError::GetI32FromWebParams {
                user_friendly: param_name.to_string(),
                developer_friendly: format!("{:?}", self.0),
                source_line_column: file_line_column(&std::panic::Location::caller()),
            }
        })?;
        Ok(value)
    }
}
