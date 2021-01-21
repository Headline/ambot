pub fn getapp_shortname(appid : i32) -> &'static str {
    return match appid {
        740 => "csgo",
        232250 => "tf2",
        232330 => "css",
        222860 => "l4d2",
        _ => "unknown",
    }
}

pub fn get_appid(shortname : &str) -> i32 {
    return match shortname {
        "csgo" => 740,
        "tf2" => 232250,
        "css" => 232330,
        "l4d2" => 222860,
        _ => 0
    }
}