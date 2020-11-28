use serde::Deserialize;

extern crate serde;

#[derive(Debug, Deserialize, PartialEq)]
pub struct RSS {
    pub channel: Channel,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Channel {
    pub title : String,
    pub link : String,
    pub description : String,
    pub language : String,
    #[serde(rename = "lastBuildDate")]
    pub last_build_date : String,
    pub generator : String,
    pub ttl : i32,
    pub image : Image,
    #[serde(rename = "item")]
    pub items : Vec<Item>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Image {
    pub url : String,
    pub title : String,
    pub link : String
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Item {
    pub title : String,
    pub link : String,
    #[serde(rename = "pubDate")]
    pub pub_date : String,
    pub description : String,
    #[serde(rename = "content:encoded", default)]
    pub content_encoded : String,
    pub category : String,
    #[serde(rename = "dc:creator", default)]
    pub author : String,
    pub guid : String
}

