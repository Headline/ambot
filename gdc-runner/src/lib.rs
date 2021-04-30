use std::fs::File;
use std::collections::HashMap;
use std::process::ExitStatus;

use crate::downloader::GDCError;
use crate::gameinfo::*;

pub mod gdcrunner;
pub mod downloader;
pub mod gameinfo;


pub struct GDCManager {
    game : Game,
    sourcemod_dir : String,
    downloads_dir : String,
    depotdownloader_path : String,
}

impl GDCManager {
    pub fn new(game : Game, sourcemod_dir : &str, downloads_dir : &str, depotdownloader_path : &str) -> GDCManager {
        GDCManager {
            game,
            sourcemod_dir : sourcemod_dir.to_owned(),
            downloads_dir : downloads_dir.to_owned(),
            depotdownloader_path : depotdownloader_path.to_owned(),
        }
    }

    pub async fn download_game(&self) -> Result<ExitStatus, downloader::GDCError> {
        let dl = downloader::DepotDownloader::new(&self.downloads_dir, self.game);
        dl.download(&self.depotdownloader_path).await
    }

    pub async fn check_gamedata(&self, output_file : &mut  File, gamedata : Vec<crate::gdcrunner::GameData>) -> HashMap<String, Result<bool, GDCError>> {
        let runner = gdcrunner::GDCRunner::load(self.game, &self.sourcemod_dir, &self.downloads_dir, gamedata);
        runner.run(output_file)
    }
}