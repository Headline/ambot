use serenity::prelude::{TypeMap, TypeMapKey};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use serenity::model::id::UserId;
use std::error::Error;

/** Caching **/
pub struct BotInfo;
impl TypeMapKey for BotInfo {
    type Value = Arc<tokio::sync::RwLock<HashMap<&'static str, String>>>;
}

pub async fn fill(
    data: Arc<tokio::sync::RwLock<TypeMap>>,
    prefix: &str,
    id: &UserId,
) -> Result<(), Box<dyn Error>> {
    let mut data = data.write().await;

    // Lets map some common things in BotInfo
    let mut map = HashMap::<&str, String>::new();
    map.insert("PLUGIN_CHANNEL", env::var("PLUGIN_CHANNEL")?);
    map.insert("BOT_PREFIX", String::from(prefix));
    map.insert("BOT_ID", id.to_string());
    data.insert::<BotInfo>(Arc::new(tokio::sync::RwLock::new(map)));

    Ok(())
}
