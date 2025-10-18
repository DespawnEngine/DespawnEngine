use std::fs;
use std::path::Path;
use serde::de::DeserializeOwned;

pub fn load_json5_dir<T: DeserializeOwned>(dir: &str) -> Vec<T> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json5") ||
                path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    match serde_json5::from_str::<T>(&content) {
                        Ok(obj) => results.push(obj),
                        Err(e) => eprintln!("Failed to parse {:?}: {e}", path),
                    }
                }
            }
        }
    }

    results
}
