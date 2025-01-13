use std::collections::HashMap;

pub struct Query {
    command: String,
    arguments: HashMap<String, String>,
}
