use std::{
    env, io, fs::File, collections::HashMap,
    path::Path, process::{Command, Stdio}
};

use walkdir::WalkDir;

use crate::downloader::GDCError;
use crate::gameinfo::*;
use std::io::Write;

#[derive(Debug)]
pub struct GameData {
    pub appid : i32,
    pub url : String,
    pub path : String
}

pub struct GDCRunner {
    sourcemod : String,
    gdc_location : String,
    gamedata_files : Vec<String>,
    download_path : String,
    game : Game
}

impl GDCRunner {
    pub async fn load(game : Game, sourcemod : &str, dl_path : &str, gamedata : Vec<GameData>) -> GDCRunner {
        // load gamedata
        let mut vec = Vec::new();
        for data in gamedata {
            if !data.path.is_empty() {
                vec.push(format!("{}/gamedata/{}", sourcemod, data.path))
            }
            else if !data.url.is_empty() {
                let filename = data.url[data.url.rfind('/').unwrap()+1..].to_string();
                let response = reqwest::get(&data.url).await;
                if let Ok(r) = response {
                    if let Ok(text) = r.bytes().await {
                        let write_path = format!("gamedata/{}", &filename);
                        if let Ok(mut file) = File::create(&write_path) {
                            let _ = file.write_all(&text);
                        }
                        let path = std::fs::canonicalize(write_path).unwrap().to_string_lossy().to_string();
                        vec.push(path);
                    }
                }
            }
            else {
                panic!("Invalid entry for load gamedata discovered.");
            }
        }

        let runner = GDCRunner {
            sourcemod : sourcemod.to_owned(),
            gdc_location : format!("{}/tools/gdc-psyfork/Release/gdc", sourcemod),
            gamedata_files : vec,
            download_path : format!("{}/{}", dl_path.to_owned(), game.appid),
            game
        };
        runner
    }

    pub fn find_binary(&self, filename : &str, try_srv : bool) -> String {
        if try_srv { // try with srv suffix
            let res = self.find_binary(&filename.replace(".so", "_srv.so"), false);
            if !res.is_empty() {
                return res;
            }
        }

        for entry in WalkDir::new(&self.download_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
            let f_name = entry.file_name().to_string_lossy();
            if f_name == filename {
                let str = entry.path().to_string_lossy().to_string();
                println!("Found file: {}", str);

                return str
            }
        }

        println!("Unable to find {}", filename);
        return String::default()
    }

    pub fn run(&self, output_file : &mut File) -> HashMap<String, Result<bool, GDCError>> {
        let cwd = env::current_dir().unwrap();
        let libpath1 = cwd.join(Path::new(&self.download_path).join(&self.game.gamedir).join("bin")).to_string_lossy().to_string();
        let libpath2 = cwd.join(Path::new(&self.download_path).join("bin")).to_string_lossy().to_string();

        let lin_server_bin = self.find_binary("server.so", true);
        let win_server_bin = self.find_binary("server.dll", false);
        let lin_engine_bin = self.find_binary("engine.so", true);
        let win_engine_bin = self.find_binary("engine.dll", false);

        let mut map = HashMap::new();
        for file in &self.gamedata_files {
            let child = Command::new(&self.gdc_location)
                .env("LD_LIBRARY_PATH", format!("{}:{}:/usr/lib", libpath2, libpath1))
                .arg("-e")
                .arg(&self.game.engine)
                .arg("-g")
                .arg(&self.game.gamedir)
                .arg("-f")
                .arg(&file)
                .arg("-b")
                .arg(&lin_server_bin)
                .arg("-w")
                .arg(&win_server_bin)
                .arg("-x")
                .arg(&lin_engine_bin)
                .arg("-y")
                .arg(&win_engine_bin)
                .arg("-s")
                .arg(&format!("{}/tools/gdc-psyfork/symbols.txt", &self.sourcemod))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match child {
                Ok(mut t) => {
                    let mut err = t.stderr.take().unwrap();
                    let mut out = t.stdout.take().unwrap();
                    io::copy(&mut err, output_file).unwrap();
                    io::copy(&mut out, output_file).unwrap();

                    map.insert(file.clone(), Ok(t.wait().unwrap().success()));
                }
                Err(e) => {
                    map.insert(file.clone(), Err(GDCError::new(&format!("{}", e))));
                }
            }
        }
        map
    }
}