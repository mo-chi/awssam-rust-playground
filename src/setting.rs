use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Global {
    pub table_name: String,
}

#[allow(unused)]
pub static GLOBAL: Lazy<Global> = Lazy::new(|| Global {
    table_name: std::env::var("DYNAMODB_TABLE_NAME").unwrap().to_string(),
});
