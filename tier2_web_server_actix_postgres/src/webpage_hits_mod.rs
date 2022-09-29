//! webpage_hits_mod.rs
// type aliases: for less verbose types and better readability of the code
type DbPool = actix_web::web::Data<deadpool_postgres::Pool>;
type ResultResponse = actix_web::Result<actix_web::HttpResponse>;
type WebQuery = actix_web::web::Query<Vec<(String, String)>>;

use crate::actix_mod::get_i32_from_web_query;
use crate::actix_mod::get_str_from_web_query;
use crate::actix_mod::return_response_no_cache;
use crate::error_mod::time_epoch;
use crate::postgres_mod::run_sql_select_query;
use crate::postgres_mod::run_sql_single_row_function;
use crate::postgres_mod::run_sql_void_function;

/// scoped actix routing near the implementation code
/// actix have this "magic data extraction thing" to accommodate different parameters
pub fn config_route_webpage_hits(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(webpage_hits_list)
        .service(webpage_hits_new)
        .service(webpage_hits_edit)
        .service(webpage_hits_insert)
        .service(webpage_hits_read)
        .service(webpage_hits_update)
        .service(webpage_hits_delete);
}

/// list all webpages and counts
#[actix_web::get("/webpage_hits_list")]
pub async fn webpage_hits_list(db_pool: DbPool) -> ResultResponse {
    println!("{} webpage_hits_list", time_epoch());

    // region: 1. parse params from web query
    // endregion

    // region: 2. send parameters to database and get data from database
    let row_set = run_sql_select_query(
        db_pool,
        "SELECT id, webpage, hit_count from webpage_hits;",
        &[],
    )
    .await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    let body = r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_hits_list</h1>
    <div class="table">
      <div></div>
      <div></div>
      <div>id</div>
      <div>webpage</div>
      <div>hit_count</div>
      {table_content}
    </div>
    <br/>
    <div>
      <button onclick="location.href='webpage_hits_new'" >New record</button>
    </div>
  </body>
</html>
"#;
    // endregion

    // region: 4. mix presentation and data because of html
    let mut table_content = String::new();
    for r in row_set {
        let id: i32 = r.get(0);
        let webpage: String = r.get(1);
        let hit_count: i32 = r.get(2);
        table_content.push_str(&format!(
            r##"
<div>
  <a class="button" href='webpage_hits_edit?id={id}'>edit</a>
</div> 
<div>
  <a class="button" href='webpage_hits_delete?id={id}'>delete</a>
</div>
<div>
  <a href='webpage_hits_read?id={id}'>{id}</a>
</div>
<div>{webpage}</div>
<div>{hit_count}</div>
"##
        ));
    }
    let body = body.replace("{table_content}", &table_content);
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}

/// new record UI
#[actix_web::get("/webpage_hits_new")]
pub async fn webpage_hits_new(db_pool: DbPool) -> ResultResponse {
    println!("{} webpage_hits_new", time_epoch());

    // region: 1. parse params from web query
    // endregion

    // region: 2. send parameters to database and get data from database
    let single_row = run_sql_single_row_function(db_pool, "webpage_hits_new", &[]).await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_hits_new</h1>
    <form action="webpage_hits_insert" method="get" >
      <p>
        <label for="id">Id:</label>
        <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
      </p>
      <p>
        <label for="id">Webpage:</label>
        <input type="text" id="webpage" name="webpage" value="{webpage}" />
      </p>
      <p>
        <label for="id">Count:</label>
        <input type="text" id="hit_count" name="hit_count" value="{hit_count}" />
      </p>
      <input type="submit" class="button" value="Submit">
      <button type="button" onclick="location.href='webpage_hits_list'" >Cancel</button>
    </form>
  </body>
</html>
"#,
    );
    // endregion

    // region: 4. mix presentation and data because of html
    let id: i32 = single_row.get(0);
    let webpage: String = single_row.get(1);
    let hit_count: i32 = single_row.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}

/// edit record UI
#[actix_web::get("/webpage_hits_edit")]
pub async fn webpage_hits_edit(db_pool: DbPool, query: WebQuery) -> ResultResponse {
    println!("{} webpage_hits_edit", time_epoch());

    // region: 1. parse params from web query
    let id = get_i32_from_web_query(&query, "id")?;
    // endregion

    // region: 2. send parameters to database and get data from database
    let single_row = run_sql_single_row_function(db_pool, "webpage_hits_read", &[&id]).await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_his_edit</h1>
    <form action="webpage_hits_update" method="get" >
      <p>
        <label for="id">Id:</label>
        <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
      </p>
      <p>
        <label for="id">Webpage:</label>
        <input type="text" id="webpage" name="webpage" value="{webpage}" />
      </p>
      <p>
        <label for="id">Hit_count:</label>
        <input type="text" id="hit_count" name="hit_count" value="{hit_count}" />
      </p>
      <button type="submit" class="button" value="Submit">Submit</button>
      <button type="button" class="button" onclick="location.href='webpage_hits_list'" >Cancel</button>
    </form>
  </body>
</html>
    "#,
    );
    // endregion

    // region: 4. mix presentation and data because of html
    let id: i32 = single_row.get(0);
    let webpage: String = single_row.get(1);
    let hit_count: i32 = single_row.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}

/// CRUD - create(insert)
#[actix_web::get("/webpage_hits_insert")]
pub async fn webpage_hits_insert(db_pool: DbPool, query: WebQuery) -> ResultResponse {
    println!("{} webpage_hits_insert", time_epoch());

    // region: 1. parse params from web query
    let id = get_i32_from_web_query(&query, "id")?;
    let webpage = get_str_from_web_query(&query, "webpage")?.to_string();
    let hit_count = get_i32_from_web_query(&query, "hit_count")?;
    // endregion

    // region: 2. send parameters to database and get data from database
    let single_row =
        run_sql_single_row_function(db_pool, "webpage_hits_insert", &[&id, &webpage, &hit_count])
            .await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_hits_insert</h1>
    <p>Record inserted!</p>
    <form>
      <p>
        <label for="id">Id:</label>
        <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
      </p>
      <p>
        <label for="id">Webpage:</label>
        <input type="text" id="webpage" name="webpage" readonly="readonly" value="{webpage}" />
      </p>
      <p>
        <label for="id">Count:</label>
        <input type="text" id="hit_count" name="hit_count" readonly="readonly" value="{hit_count}" />
      </p>
    </form>
    <div>
      <button onclick="location.href='webpage_hits_list'" >List</button>
    </div>
  </body>
</html>
    "#,
    );
    // endregion

    // region: 4. mix presentation and data because of html
    let id: i32 = single_row.get(0);
    let webpage: String = single_row.get(1);
    let hit_count: i32 = single_row.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}

/// CRUD - read one record
#[actix_web::get("/webpage_hits_read")]
pub async fn webpage_hits_read(db_pool: DbPool, query: WebQuery) -> ResultResponse {
    println!("{} webpage_hits_read", time_epoch());

    // region: 1. parse params from web query
    let id = get_i32_from_web_query(&query, "id")?;
    // endregion

    // region: 2. send parameters to database and get data from database
    let single_row = run_sql_single_row_function(db_pool, "webpage_hits_read", &[&id]).await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_hits_read</h1>
    <form >
      <p>
        <label for="id">Id:</label>
        <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
      </p>
      <p>
        <label for="id">Webpage:</label>
        <input type="text" id="webpage" name="webpage" readonly="readonly" value="{webpage}" />
      </p>
      <p>
        <label for="id">Count:</label>
        <input type="text" id="hit_count" name="hit_count" readonly="readonly" value="{hit_count}" />
      </p>
    </form>
    <div>
      <button onclick="location.href='webpage_hits_list'" >List</button>
    </div>
  </body>
</html>
    "#,
    );
    // endregion

    // region: 4. mix presentation and data because of html
    let id: i32 = single_row.get(0);
    let webpage: String = single_row.get(1);
    let hit_count: i32 = single_row.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}

/// CRUD - update
#[actix_web::get("/webpage_hits_update")]
pub async fn webpage_hits_update(db_pool: DbPool, query: WebQuery) -> ResultResponse {
    println!("{} webpage_hits_update", time_epoch());

    // region: 1. parse params from web query
    let id = get_i32_from_web_query(&query, "id")?;
    let webpage = get_str_from_web_query(&query, "webpage")?.to_string();
    let hit_count = get_i32_from_web_query(&query, "hit_count")?;
    // endregion

    // region: 2. send parameters to database and get data from database
    let single_row =
        run_sql_single_row_function(db_pool, "webpage_hits_update", &[&id, &webpage, &hit_count])
            .await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_hits_update</h1>
    <p>Record updated!</p>
    <form >
      <p>
        <label for="id">Id:</label>
        <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
      </p>
      <p>
        <label for="id">Webpage:</label>
        <input type="text" id="webpage" name="webpage" readonly="readonly" value="{webpage}" />
      </p>
      <p>
        <label for="id">Count:</label>
        <input type="text" id="hit_count" name="hit_count" readonly="readonly" value="{hit_count}" />
      </p>
    </form>
    <div>
      <button onclick="location.href='webpage_hits_list'" >List</button>
    </div>
  </body>
</html>
    "#,
    );
    // endregion

    // region: 4. mix presentation and data because of html
    let id: i32 = single_row.get(0);
    let webpage: String = single_row.get(1);
    let hit_count: i32 = single_row.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}

/// CRUD - delete
#[actix_web::get("/webpage_hits_delete")]
pub async fn webpage_hits_delete(db_pool: DbPool, query: WebQuery) -> ResultResponse {
    println!("{} webpage_hits_delete", time_epoch());

    // region: 1. parse params from web query
    let id = get_i32_from_web_query(&query, "id")?;
    // endregion

    // region: 2. send parameters to database and get data from database
    run_sql_void_function(db_pool, "webpage_hits_delete", &[&id]).await?;
    // endregion

    // region: 3. presentation with replaceable mustache
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
  <head>
    <link rel="stylesheet" href="/webpage_hits_admin/css/webpage_hits_admin.css" />
  </head>
  <body>
    <h1>webpage_hits_delete</h1>
    <p>Record deleted!</p>
    <div>
      <button onclick="location.href='webpage_hits_list'" >Return to list</button>
    </div>
  </body>
</html>
"#,
    );
    // endregion

    // region: 4. mix presentation and data because of html
    // endregion

    // region: 5. return response
    return_response_no_cache(body)
    // endregion
}
