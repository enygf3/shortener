use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::http::StatusCode;
use rand::{distr::Alphanumeric, Rng, RngExt};

#[derive(Clone, Default)]
pub(crate) struct Store {
    state: Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub(crate) fn new() -> Self {
        Self { state: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub(crate) async fn get_url(&self, url: String) -> Option<String> {
        println!("GET {}", url);
        self.state.lock().unwrap().get(&url).cloned()
    }

    pub(crate) async fn put_url(&self, url: String) -> (StatusCode, String) {
        let mut state = self.state.lock().unwrap();
        let rand_str = Self::create_random_url();
        state.insert(rand_str.clone(), url.clone());
        println!("PUT {}", rand_str);
        println!("PUT {}", url);
        (StatusCode::OK, rand_str)
    }

    fn create_random_url() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }
}