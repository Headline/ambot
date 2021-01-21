use std::{
    env, io, fs::File, collections::HashMap,
    path::Path, process::{Command, Stdio}
};

use walkdir::WalkDir;

use crate::appid_translator::getapp_shortname;
use crate::downloader::GDCError;

pub struct GDCRunner {
    sourcemod : String,
    gdc_location : String,
    gamedata_files : Vec<String>,
    shortname : String,
    download_path : String
}

impl GDCRunner {
    pub fn load(sourcemod : &str, appid : i32, dl_path : &str) -> GDCRunner {
        // load gamedata
        let mut vec = Vec::new();
        let shortname = getapp_shortname(appid);
        for entry in WalkDir::new(format!("{}/gamedata", sourcemod))
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
            let filename = entry.path().to_string_lossy().to_string();
            if filename.contains(shortname) {
                vec.push(filename);
            }
        }

        let runner = GDCRunner {
            sourcemod : sourcemod.to_owned(),
            gdc_location : format!("{}/tools/gdc-psyfork/Release/gdc", sourcemod),
            gamedata_files : vec,
            shortname : shortname.to_owned(),
            download_path : dl_path.to_owned()
        };
        runner
    }

    pub fn run(&self, output_file : &mut File) -> HashMap<String, Result<bool, GDCError>> {
        let cwd = env::current_dir().unwrap();
        let libpath1 = cwd.join(Path::new(&self.download_path).join(&self.shortname).join("bin")).to_string_lossy().to_string();
        let libpath2 = cwd.join(Path::new(&self.download_path).join("bin")).to_string_lossy().to_string();

        let mut map = HashMap::new();
        for file in &self.gamedata_files {
            let child = Command::new(&self.gdc_location)
                .env("LD_LIBRARY_PATH", format!("{}:{}", libpath2, libpath1))
                .arg("-e")
                .arg(&self.shortname)
                .arg("-g")
                .arg(&self.shortname)
                .arg("-f")
                .arg(&file)
                .arg("-b")
                .arg(&format!("{}/{}/bin/server.so", &self.download_path, &self.shortname))
                .arg("-w")
                .arg(&format!("{}/{}/bin/server.dll", &self.download_path, &self.shortname))
                .arg("-x")
                .arg(&format!("{}/bin/engine.so", &self.download_path))
                .arg("-y")
                .arg(&format!("{}/bin/engine.dll", &self.download_path))
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