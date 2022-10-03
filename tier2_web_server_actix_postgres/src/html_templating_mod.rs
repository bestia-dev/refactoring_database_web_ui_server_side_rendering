// html_templating_mod.rs

/// struct fields scope and function name are used as the folder and file name for the template
pub fn read_template(scope: &str, name: &str) -> String {
    let path = format!("webpage_hits_admin/{}/{}.html", scope, name);
    std::fs::read_to_string(path).unwrap()
}

/// simple replace placeholders with values from Row
/// inside the template there are delimited variables like: {variable}
/// the data comes from the postgres database inside a Row struct
/// panics if the type is not recognized
pub fn template_replace_fields_from_single_row(
    text_with_placeholders: &str,
    single_row: tokio_postgres::Row,
) -> String {
    let mut text_replaced = text_with_placeholders.to_string();
    for (i, col) in single_row.columns().iter().enumerate() {
        let placeholder = String::new() + "{" + col.name() + "}";
        // postgres has a very long list of types
        // I will use only a small subset of them
        use tokio_postgres::types::Type;
        match col.type_() {
            &Type::TEXT | &Type::VARCHAR => {
                let value: String = single_row.get(i);
                let value = html_escape::encode_text(&value);
                text_replaced = text_replaced.replace(&placeholder, &value)
            }
            &Type::INT4 => {
                let value: i32 = single_row.get(i);
                text_replaced = text_replaced.replace(&placeholder, &value.to_string())
            }
            &Type::VOID => (),
            _ => panic!("Unrecognized postgres type: {:?}", col.type_()),
        }
    }
    text_replaced
}
