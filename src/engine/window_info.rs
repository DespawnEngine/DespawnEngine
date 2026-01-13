use std::sync::{Arc, OnceLock};

use egui::mutex::Mutex;

use crate::arguments::Backends;

#[derive(Clone)]
pub struct WindowInfo {
    pub resolution: Arc<Mutex<Option<[u32; 2]>>>,
    pub title: Arc<Mutex<Option<String>>>,
    pub backend: Arc<Mutex<Option<Backends>>>,
}

impl WindowInfo {
    fn new() -> Self {
        Self {
            resolution: Arc::new(Mutex::new(None)),
            title: Arc::new(Mutex::new(None)),
            backend: Arc::new(Mutex::new(None)),
        }
    }

    pub fn instance() -> Self {
        static INSTANCE: OnceLock<WindowInfo> = OnceLock::new();
        INSTANCE.get_or_init(WindowInfo::new).clone()
    }

    pub fn set_resolution(new_resolution: [u32; 2]) {
        *Self::instance().resolution.lock() = Some(new_resolution);
    }
    pub fn set_title(title: String) {
        *Self::instance().title.lock() = Some(title);
    }
    pub fn set_backend(backend: Backends) {
        *Self::instance().backend.lock() = Some(backend);
    }
}
