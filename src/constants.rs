pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

#[derive(Debug, Clone)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    pub blocknative_api_key: String,
}

impl Env {
    pub fn new() -> Self {
        Env {
            https_url: get_env("HTTPS_URL"),
            wss_url: get_env("WSS_URL"),
            blocknative_api_key: get_env("BLOCKNATIVE_API_KEY"),
        }
    }
}
