use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}
