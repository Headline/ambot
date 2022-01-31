#[derive(Clone)]
pub struct Game {
    pub appid : i32,
    pub name : String,
    pub gamedir : &'static str,
    pub engine : &'static str,
}

pub struct GameCache {
    cache : Vec<Game>
}
impl GameCache {
    pub fn new() -> GameCache {
        GameCache {
            cache: vec! [
                Game { appid: 740, name: String::from("csgo"), gamedir: "csgo", engine: "csgo" },
                Game { appid: 232250, name: String::from("tf2"), gamedir: "tf", engine: "orangebox_valve" },
                Game { appid: 232330, name: String::from("css"), gamedir: "cstrike", engine: "css" },
                Game { appid: 222860, name: String::from("l4d2"), gamedir: "left4dead2", engine: "left4dead2" },
                Game { appid: 317670, name: String::from("nmrih"), gamedir: "nmrih", engine: "sdk2013" },
            ]
        }
    }

    pub fn get_ids(&self) -> Vec<i32> {
        self.cache.iter().map(|p| p.appid).collect()
    }

    pub fn lookup_shortname(&self, shortname : &str) -> Option<&Game> {
        self.cache.iter().find(|p| p.name == shortname)
    }
    pub fn lookup_appid(&self, appid : i32) -> Option<&Game> {
        self.cache.iter().find(|p| p.appid == appid)
    }
}