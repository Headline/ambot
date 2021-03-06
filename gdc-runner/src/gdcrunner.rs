use std::{
    env, io, fs::File, collections::HashMap,
    path::Path, process::{Command, Stdio}
};

use walkdir::WalkDir;

use crate::downloader::GDCError;
use crate::gameinfo::*;

pub struct GDCRunner {
    sourcemod : String,
    gdc_location : String,
    gamedata_files : Vec<String>,
    download_path : String,
    game : Game
}

impl GDCRunner {
    pub fn load(game : Game, sourcemod : &str, dl_path : &str) -> GDCRunner {
        // load gamedata
        let mut vec = Vec::new();
        for entry in WalkDir::new(format!("{}/gamedata", sourcemod))
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
            let filename = entry.path().to_string_lossy().to_string();
            if filename.contains(game.name) {
                vec.push(filename);
            }
        }

        let runner = GDCRunner {
            sourcemod : sourcemod.to_owned(),
            gdc_location : format!("{}/tools/gdc-psyfork/Release/gdc", sourcemod),
            gamedata_files : vec,
            download_path : dl_path.to_owned(),
            game
        };
        runner
    }

    pub fn find_binary(&self, filename : &str) -> String {
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
        String::default()
    }
    pub fn run(&self, output_file : &mut File) -> HashMap<String, Result<bool, GDCError>> {
        let cwd = env::current_dir().unwrap();
        let libpath1 = cwd.join(Path::new(&self.download_path).join(&self.game.gamedir).join("bin")).to_string_lossy().to_string();
        let libpath2 = cwd.join(Path::new(&self.download_path).join("bin")).to_string_lossy().to_string();


        let mut map = HashMap::new();
        for file in &self.gamedata_files {
            let child = Command::new(&self.gdc_location)
                .env("LD_LIBRARY_PATH", format!("{}:{}", libpath2, libpath1))
                .arg("-e")
                .arg(&self.game.name)
                .arg("-g")
                .arg(&self.game.name)
                .arg("-f")
                .arg(&file)
                .arg("-b")
                .arg(&self.find_binary("server.so"))
                .arg("-w")
                .arg(&self.find_binary("server.dll"))
                .arg("-x")
                .arg(&self.find_binary("engine.so"))
                .arg("-y")
                .arg(&self.find_binary("engine.dll"))
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