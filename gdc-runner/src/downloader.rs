use crate::appid_translator::getapp_shortname;
use std::process::{Command, ExitStatus, Stdio};
use std::path::Path;
use core::fmt;

#[derive(Debug, Clone)]
pub struct GDCError {
    message : String
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
}

impl DepotDownloader {
    pub fn new(downloads_dir : &str) -> DepotDownloader {
        DepotDownloader {
            downloads_dir : downloads_dir.to_owned()
        }
    }

    pub fn get_download_path(&self, appid : i32) -> String {
        let shortname = getapp_shortname(appid);
        if shortname != "unknown" {
            Path::new(&self.downloads_dir).join(shortname).to_str().unwrap().to_owned()
        }
        else {
            String::new()
        }
    }

    pub async fn download(&self, appid : i32, depotdownloader_path : &str) -> Result<ExitStatus, GDCError> {
        let download_directory = self.get_download_path(appid);
        if download_directory.is_empty() {
            return Err(GDCError::new("Unsupported appid"));
        }

        if std::fs::create_dir(&download_directory).is_err() {
            if !Path::new(&download_directory).exists() {
                return Err(GDCError::new("Unable to create download directory!"));
            }
        }

        let execution_result = DepotDownloader::spawn_process(appid, &download_directory, depotdownloader_path).await;
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
            .stdout(Stdio::null())
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