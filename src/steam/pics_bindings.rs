use std::collections::HashMap;

use serde::*;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct PicsResponse {
    pub success : i32,
    pub apps: HashMap<String, App>, // string is id
}
#[derive(Clone, Debug, Deserialize, Default)]
pub struct Depots {
    pub branches :HashMap<String, Branch>
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Branch {
    pub build_id : String,
    pub description : String
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct App {
    pub appid : String,
    pub change_number : u64,
    pub common : Common,
    pub depots : Depots,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Common {
    pub name : String
}