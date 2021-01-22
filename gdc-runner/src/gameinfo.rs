#[derive(Copy, Clone)]
pub struct Game {
    pub appid : i32,
    pub name : &'static str,
    pub gamedir : &'static str
}

pub struct GameCache {
    cache : Vec<Game>
}
impl GameCache {
    pub fn new() -> GameCache {
        GameCache {
            cache: vec! [
                Game { appid: 740, name: "csgo", gamedir: "csgo" },
                Game { appid: 232250, name: "tf2", gamedir: "tf2" },
                Game { appid: 232330, name: "css", gamedir: "cstrike" },
                Game { appid: 222860, name: "l4d2", gamedir: "left4dead2" },
            ]
        }
    }

    pub fn lookup_shortname(&self, shortname : &str) -> Option<&Game> {
        self.cache.iter().find(|p| p.name == shortname)
    }
    pub fn lookup_appid(&self, appid : i32) -> Option<&Game> {
        self.cache.iter().find(|p| p.appid == appid)
    }
}