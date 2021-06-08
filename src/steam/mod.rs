pub mod pics_bindings;

use std::collections::HashMap;
use crate::steam::pics_bindings::{PicsResponse, App};
use std::future::Future;
use std::sync::Arc;
use serenity::prelude::TypeMap;
use tokio::sync::RwLock;
use serenity::CacheAndHttp;
use gdcrunner::gameinfo::GameCache;

pub struct GameVersionManager {
    cache : GameCache,
    apps : HashMap<u64, u64> // appid, change_number
}

impl GameVersionManager {
    pub fn new() -> Self {
        GameVersionManager {
            cache: GameCache::new(),
            apps: Default::default()
        }
    }

    pub fn get_apps(&self) -> Vec<i32> {
        self.cache.get_ids()
    }

    // true if game has updated
    pub fn check_update(&mut self, id : u64, new_number : u64) -> bool {
        if let Some(old_num) = self.apps.insert(id, new_number) {
            if old_num != 0 && old_num != new_number {
                return true
            }
        }
        return false
    }
}

pub fn start_polling<F: 'static, Fut>(data: Arc<RwLock<TypeMap>>, http : Arc<CacheAndHttp>, on_update: F)
    where
        F: Fn(Arc<RwLock<TypeMap>>, Arc<CacheAndHttp>, u64, App) -> Fut + Send + Sync,
        Fut: Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        let mut n = GameVersionManager::new();
        let client = reqwest::Client::new();

        loop {
            let apps : Vec<String> = n.get_apps().iter().map(|&id| id.to_string()).collect();

            let apps_str = apps.join(",");
            let endpoint = format!("http://127.0.0.1:23455/info?apps={}", apps_str);

            let response = client.get(&endpoint).send().await.unwrap();
            let results = response.json::<PicsResponse>().await.unwrap();
            for (k, v) in results.apps {
                let id = k.parse::<u64>().unwrap();
                if n.check_update(id, v.depots.branches["public"].build_id.parse::<u64>().unwrap()) {
                    on_update(data.clone(), http.clone(), id, v).await;
                }
            }

            tokio::time::delay_for(core::time::Duration::new(120, 0)).await;
        }
    });
}

