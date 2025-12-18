use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::OnceLock,
};

use serde_json5;

#[derive(Clone, Copy)]
pub struct UserSettings {
    pub mouse_sensitivity: f32,
    pub vertical_render_distance: u32,
    pub horizontal_render_distance: u32,
}

const DEFAULT_MOUSE_SENSITIVITY: f32 = 100.0;
const DEFAULT_RENDER_DISTANCE: u32 = 2;

impl UserSettings {
    pub fn instance() -> Self {
        static INSTANCE: OnceLock<UserSettings> = OnceLock::new();
        *INSTANCE.get_or_init(UserSettings::new)
    }

    pub fn new() -> Self {
        let settings_file_path: Vec<&Path> = vec![Path::new("settings.json5")];
        let used_settings_file_path = std::path::absolute(
            settings_file_path
                .first()
                .expect("failed to get any settings file paths"),
        )
        .unwrap();

        let file: File = File::open(&used_settings_file_path)
            .unwrap_or_else(|_| panic!("failed to open file {used_settings_file_path:?}"));

        let buf_reader = BufReader::new(file);

        let data: HashMap<String, String> = serde_json5::from_reader(buf_reader)
            .unwrap_or_else(|_| panic!("failed to parse file {used_settings_file_path:?}"));

        let mouse_sensitivity: f32 = data
            .get("Mouse Sensitivity")
            .unwrap_or(&DEFAULT_MOUSE_SENSITIVITY.to_string())
            .parse::<f32>()
            .unwrap_or_else(|_| -> f32 {
                println!(
                    "failed to parse mouse_sensitivity from settings file {used_settings_file_path:?}"
                );
                DEFAULT_MOUSE_SENSITIVITY
            });

        let horizontal_render_distance: u32 = data
            .get("Horizonal Render Distance")
            .unwrap_or(&DEFAULT_RENDER_DISTANCE.to_string())
            .parse::<u32>()
            .unwrap_or_else(|_| -> u32 {
                println!(
                    "failed to parse horizontal_render_distance from settings file {used_settings_file_path:?}"
                );
                DEFAULT_RENDER_DISTANCE
            });

        let vertical_render_distance : u32 = data
            .get("Vertical Render Distance")
            .unwrap_or(&DEFAULT_RENDER_DISTANCE.to_string())
            .parse::<u32>()
            .unwrap_or_else(|_| -> u32 {
                println!(
                    "failed to parse vertical_render_distance from settings file {used_settings_file_path:?}"
                );
                DEFAULT_RENDER_DISTANCE
            });

        UserSettings {
            mouse_sensitivity,
            vertical_render_distance,
            horizontal_render_distance,
        }
    }
}
