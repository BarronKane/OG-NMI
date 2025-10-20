use serde::{Deserialize, Serialize};
use std::sync::RwLock;

static SECRETS_CACHE: RwLock<Option<Secrets>> = RwLock::new(None);

#[derive(Serialize, Deserialize, Clone)]
pub struct Secrets {
    pub guild_id: u64,
    pub token: String,

    pub authorized_ids: Vec<u64>
}

impl Secrets {
    pub fn get_secrets() -> Secrets {
        // Try to read from cache first.
        if let Ok(cache) = SECRETS_CACHE.read() {
            if let Some(secrets) = cache.as_ref() {
                return secrets.clone();
            }
        }
        
        // Cache miss - load from disk.
        let file = std::fs::File::open("secrets.json").expect("secrets.json not found");
        let secrets: Secrets = serde_json::from_reader(file).expect("secrets.json not valid");
        
        // Update cache.
        if let Ok(mut cache) = SECRETS_CACHE.write() {
            *cache = Some(secrets.clone());
        }
        
        secrets
    }
    
    pub fn save_secrets(&self) {
        let file = std::fs::File::create("secrets.json").expect("Secrets.json couldn't be opened.");
        serde_json::to_writer(file, &self).expect("Failed to write secrets to file");
        
        // Update cache with new values.
        if let Ok(mut cache) = SECRETS_CACHE.write() {
            *cache = Some(self.clone());
        }
    }
}
