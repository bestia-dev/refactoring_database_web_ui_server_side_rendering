// server_side_single_row.rs

// Structs and methods for server side rendering in web server functions
// for single row: (new, insert, show, edit, update, delete):
// 1. parse web data: strings coming from the browser in path, query and form
// 2. find out the parameters of an sql function with data types
// 3. cast the web data to call the function with correct data types
// 4. retrieve sql data as single row or void
// 5. read html template (presentation) from disk or cache
// 6. mix presentation and data, because this is server-side rendering
// 7. return a response with no cache (because data in database can change fast)


use crate::{actix_mod::WebParams, postgres_mod::FunctionName, postgres_type_mod::PostgresValue};

// type aliases: for less verbose types and better readability of the code
type DataAppState = actix_web::web::Data<crate::AppState>;
type WebForm = actix_web::web::Form<Vec<(String, String)>>;
type WebQuery = actix_web::web::Query<Vec<(String, String)>>;
type ResultResponse = actix_web::Result<actix_web::HttpResponse>;

/// the main ServerSideSingleRow object (struct with implementation)
pub struct ServerSideSingleRow<'a> {
    app_state: &'a DataAppState,
    scope: &'a str,
    function_name: FunctionName,
    web_params: WebParams,
    sql_params: Vec<PostgresValue>,
}


impl<'a> ServerSideSingleRow<'a> {
    /// constructor for the server side rendering object
    /// It takes all the data needed to execute the function
    #[track_caller]
    pub fn new(
        app_state: &'a DataAppState,
        scope: &'a str,
        function_name: &'static str,
        query: &'a WebQuery,
        form: &'a Option<WebForm>,
    ) -> ServerSideSingleRow<'a> {
        println!("{} {}", crate::error_mod::time_epoch(), function_name);
        // region: 1. parse web data: strings coming from the browser in path, query and form
        let web_params = WebParams::from_actix(query, form);
        // endregion

        ServerSideSingleRow {
            app_state,
            scope,
            function_name: FunctionName(function_name.to_string()),
            web_params,
            sql_params: vec![],
        }
    }


    /// typical steps for a web app function for single Row sql function (or void function)
    /// These steps can be called separately if some customization is needed
    pub async fn run_single_row_sql_and_process_html(&mut self) -> ResultResponse {
        // region: 2. find out the parameters of an sql function with data types
        self.prepare_function_params();
        // endregion

        // region: 3. cast the web data to call the function with correct data types
        let sql_params = self.ref_to_function_params();
        // endregion

        // region: 4. retrieve sql data as single row or void
        let single_row = self.run_sql_single_row_function(sql_params).await;
        // endregion

        // region: 5. read html template (presentation) from disk or cache
        let body = crate::html_templating_mod::read_template(self.scope, &self.function_name.0);
        // endregion

        // region: 6. mix presentation and data, because this is server-side rendering
        let body =
            crate::html_templating_mod::template_replace_fields_from_single_row(&body, single_row);
        // endregion

        // region: 7. return a response with no cache (because data in database can change fast)
        crate::actix_mod::return_response_no_cache(body)
        // endregion
    }


    /// prepares input params for sql function inside struct field sql_params
    pub fn prepare_function_params(&mut self) {
        let func_hm_name_type = self
            .app_state
            .sql_function_input_params
            .get(&self.function_name)
            .unwrap();

        for (param_name, sql_type) in func_hm_name_type.iter() {
            let name = param_name
                .0
                .trim_start_matches("_")
                .trim_start_matches("in_");
            //dbg!(&name);
            //dbg!(sql_type.as_ref());
            match sql_type.as_ref() {
                "character" => {
                    self.sql_params.push(PostgresValue::String(
                        self.web_params.get_str(name).unwrap().to_string(),
                    ));
                }
                "integer" => {
                    self.sql_params
                        .push(PostgresValue::I32(self.web_params.get_i32(name).unwrap()));
                }
                _ => panic!("sql_type is unknown: {:?}", sql_type),
            }
        }
        //dbg!(&vec_multi_type);
    }

    /// returns a reference to the values in the struct field sql_params
    /// this is the format expected by the postgres library
    pub fn ref_to_function_params(&self) -> Vec<&(dyn tokio_postgres::types::ToSql + Sync)> {
        let mut sql_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];

        for x in self.sql_params.iter() {
            match x {
                PostgresValue::String(xx) => sql_params.push(xx),
                PostgresValue::I32(xx) => sql_params.push(xx),
            }
        }
        sql_params
    }

    /// run sql single row function or void function
    /// void function also returns a Row with the data type Void
    pub async fn run_sql_single_row_function(
        &self,
        sql_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)>,
    ) -> tokio_postgres::Row {
        let postgres_client =
            crate::deadpool_mod::get_postgres_client_from_pool(&self.app_state.db_pool)
                .await
                .unwrap();

        let placeholders = crate::postgres_mod::prepare_placeholders_for_sql_params(&sql_params);
        let query = format!("SELECT * from {}({});", self.function_name.0, placeholders);
        // TODO: convert sql errors in a single place
        let row = postgres_client
            .query_one(&query, &sql_params)
            .await
            .expect(&format!("{}, {:?}", query, sql_params));

        row
    }
}
