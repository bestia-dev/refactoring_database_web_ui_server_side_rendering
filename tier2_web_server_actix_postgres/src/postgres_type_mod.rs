// postgres_types_mod.rs

// This is a subset of postgres data-types that can be used in this Rust project.
// If you need some more, just add it. If you need even more flexibility, then instead of enums,
// you implements traits. But for this simple example I don't need to complicate.
// Postgres have many different names for data-types and it is confusing.


/// PostgresInputType names as string come out of the view get_function_input_params and
/// is used for input params for functions.
/// The names are strictly in lowercase, but Rust insist the enum variant are capitalized.
/// snake case will be ok here, because we have always only 1 word
#[derive(strum::AsRefStr, strum::EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum PostgresInputType {
    /// 4 bytes
    Integer,
    /// can be in 2 words "Character varying", but I use only the first word
    Character,
    /// text (max 2GB)
    Text,
}


/// PostgresFieldType names as string come out of the view get_view_fields and
/// is used for constructing the WHERE clause.
/// The names are strictly in lowercase, but Rust insist the enum variant are capitalized.
/// snake case will be ok here, because we have always only 1 word
#[derive(strum::AsRefStr, strum::EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum PostgresFieldType {
    /// 4 bytes
    Int4,
    /// varchar
    Varchar,
    /// names of postgres objects
    Name,
    /// text (max 2GB)
    Text,
}


/// PostgresValue can contain values of different data types.
/// For this simple example this is easier then implementing traits for every type.
/// I want deliberately limit the use to just a few data types for simplicity.
#[derive(Debug)]
pub enum PostgresValue {
    String(String),
    I32(i32),
}

