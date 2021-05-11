use crate::app::App;
use std::sync::Arc;
use tokio::sync::Mutex;
use super::IOEvent;

pub struct EventHandler {
    app: Arc<Mutex<App>>,
}

impl EventHandler {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            app,
        }
    }

    pub async fn handle_event(&mut self, event: String) {
        
    }
}
