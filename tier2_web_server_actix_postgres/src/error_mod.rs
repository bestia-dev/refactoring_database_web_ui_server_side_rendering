// error_mod.rs

/// enum for library errors with thiserror
/// thiserror generates the Display trait for enum variants
/// user_friendly is for user message, developer_friendly is for developer log
#[derive(thiserror::Error, Debug)]
pub enum LibError {
    /// Database connection error.
    #[error("Database connection error.")]
    DatabaseConnection,
    /// Query error: {user_friendly}
    #[error("Query error: {user_friendly}")]
    QueryError {
        source_error: tokio_postgres::Error,
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /// The value does not exist in web query: {user_friendly}
    #[error("The value does not exist in web query: {user_friendly}")]
    GetStrFromWebParams {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /// The value is not i32: {user_friendly}
    #[error("The value is not i32: {user_friendly}")]
    GetI32FromWebParams {
        user_friendly: String,
        developer_friendly: String,
        source_line_column: String,
    },
    /*
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    */
}

/// actix error has this trait for custom errors
impl actix_web::ResponseError for LibError {
    /// html status code for error
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    /// Log the developer_friendly on the server
    /// respond the user_friendly to the user
    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        // more information for the developer
        // I need the exact time to match the user message with the log
        let time = time_epoch();
        // log is developer friendly with many more info
        log::error!("{time} {}\n{:#?}", self, self);
        // only the user-friendly error for the user
        actix_web::HttpResponse::build(status_code).body(format!("{time} {}", self))
    }
}

/// time as a big Unix epoch int
pub fn time_epoch() -> u128 {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    time
}

pub fn file_line_column(source_caller_location: &std::panic::Location) -> String {
    format!(
        "{}:{}:{}",
        source_caller_location.file(),
        source_caller_location.line(),
        source_caller_location.column()
    )
}
