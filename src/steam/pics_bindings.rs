use std::collections::HashMap;

use serde::*;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct PicsResponse {
    pub success : bool,
    pub apps: HashMap<String, App> // string is id
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct App {
    pub appid : String,
    pub change_number : u64,
    pub info : Info
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Info {
    pub name : String
}