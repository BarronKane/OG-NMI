use serde::Deserialize;

#[derive(Deserialize)]
pub struct Secrets {
    pub guild_id: u64,
    pub token: String,

    pub authorized_ids: Vec<u64>
}

pub fn get_secrets() -> Secrets {
    let file = std::fs::File::open("secrets.json").expect("secrets.json not found");
    let secrets: Secrets = serde_json::from_reader(file).expect("secrets.json not valid");
    secrets
}