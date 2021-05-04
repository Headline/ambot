use std::collections::HashMap;

use serde::*;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct PicsResponse {
    pub success : i32,
    pub apps: HashMap<String, App> // string is id
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct App {
    pub appid : String,
    pub change_number : u64,
    pub common : Common
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Common {
    pub name : String
}