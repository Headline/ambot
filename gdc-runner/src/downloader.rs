use std::process::{Command, ExitStatus, Stdio};
use std::path::Path;
use core::fmt;
use crate::gameinfo::*;

#[derive(Debug, Clone)]
pub struct GDCError {
    message : String,
}
impl GDCError {
    pub fn new(message : &str) -> GDCError {
        GDCError {
            message : message.to_string()
        }
    }
}
impl fmt::Display for GDCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct DepotDownloader {
    downloads_dir : String,
    game : Game,
}

impl DepotDownloader {
    pub fn new(downloads_dir : &str, game : Game) -> DepotDownloader {
        DepotDownloader {
            downloads_dir : downloads_dir.to_owned(),
            game
        }
    }

    pub fn get_download_path(&self) -> String {
        Path::new(&self.downloads_dir).join(String::from(&self.game.name)).to_str().unwrap().to_owned()
    }

    pub async fn download(&self,  depotdownloader_path : &str) -> Result<ExitStatus, GDCError> {
        let download_directory = self.get_download_path();

        if std::fs::create_dir(&download_directory).is_err() {
            if !Path::new(&download_directory).exists() {
                return Err(GDCError::new("Unable to create download directory!"));
            }
        }

        let execution_result = DepotDownloader::spawn_process(self.game.appid, &download_directory, depotdownloader_path).await;
        match execution_result {
            Ok(t) => Ok(t),
            Err(e) => Err(GDCError::new(&format!("{}", e)))
        }
    }

    async fn spawn_process(appid : i32, download_dir : &str, depotdownloader_path : &str) -> Result<ExitStatus, GDCError> {
        let child = Command::new("dotnet")
            .current_dir(depotdownloader_path)
            .arg("DepotDownloader.dll")
            .arg("-app")
            .arg(appid.to_string())
            .arg("-all-platforms")
            .arg("-filelist")
            .arg("files.txt")
            .arg("-dir")
            .arg(download_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        match child {
            Ok(mut t) => {
                match t.wait() {
                    Ok(exit) => Ok(exit),
                    Err(e) => Err(GDCError::new(&format!("{}", e)))
                }
            },
            Err(e) => Err(GDCError::new(&format!("{}", e)))
        }
    }
}