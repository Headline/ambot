pub fn getapp_shortname(appid : i32) -> &'static str {
    return match appid {
        740 => "csgo",
        232250 => "tf2",
        _ => "unknown",
    }
}