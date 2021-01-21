use std::fs::File;
use std::collections::HashMap;
use std::process::ExitStatus;

use crate::downloader::GDCError;

mod gdcrunner;
pub mod downloader;
mod appid_translator;


pub struct GDCManager {
    appid : i32,
    sourcemod_dir : String,
    downloads_dir : String,
    depotdownloader_path : String,
}

impl GDCManager {
    pub fn new(appid : i32, sourcemod_dir : &str, downloads_dir : &str, depotdownloader_path : &str) -> GDCManager {
        GDCManager {
            appid,
            sourcemod_dir : sourcemod_dir.to_owned(),
            downloads_dir : downloads_dir.to_owned(),
            depotdownloader_path : depotdownloader_path.to_owned(),
        }
    }

    pub async fn download_game(&self) -> Result<ExitStatus, downloader::GDCError> {
        let dl = downloader::DepotDownloader::new(&self.downloads_dir);
        dl.download(self.appid, &self.depotdownloader_path).await
    }

    pub async fn check_gamedata(&self, output_file : &mut  File) -> HashMap<String, Result<bool, GDCError>> {
        let runner = gdcrunner::GDCRunner::load(&self.sourcemod_dir, self.appid, &self.downloads_dir);
        runner.run(output_file)
    }
}