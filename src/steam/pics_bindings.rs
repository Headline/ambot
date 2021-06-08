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
    #[serde(rename = "buildid")]
    pub build_id : String,
    pub description : Option<String>
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct App {
    pub appid : String,
    pub public_only : Option<u64>, // if exists and is == 1 then we prob have no depots
    pub change_number : u64,
    pub common : Common,
    pub depots : Option<Depots>, // see public_only
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Common {
    pub name : String
}