use web_sys::Storage;
use web_sys::Window;

pub struct ClientWindow {
    pub window: web_sys::Window,
    pub local_storage: web_sys::Storage,
    pub player_id_tag: Option<String>,
    pub player_id_key: Option<String>,
    pub settings: Option<String>,
    pub day: bool,
}

impl ClientWindow {
    pub fn new() -> Result<Self, String> {
        let window: Window = Self::get_window()?;
        let local_storage: Storage = Self::get_local_storage(&window)?;
        let player_id_tag: Option<String> = Self::get_storage_item(&local_storage, "player_id_tag");
        let player_id_key: Option<String> = Self::get_storage_item(&local_storage, "player_id_key");
        let settings: Option<String> = Self::get_storage_item(&local_storage, "player_settings");
        let day: bool = Self::get_day_state(&local_storage)?;
        Ok(Self {
            window,
            local_storage,
            player_id_tag,
            player_id_key,
            settings,
            day,
        })
    }

    fn get_window() -> Result<Window, String> {
        match web_sys::window() {
            Some(window) => Ok(window),
            None => Err(
                "utils_files/src/window_state.rs: new(): Window object not found; \n\t".to_string(),
            ),
        }
    }

    fn get_local_storage(window: &Window) -> Result<Storage, String> {
        match window.local_storage() {
            Ok(option) => match option {
                Some(storage) => Ok(storage),
                None => Err(
                    "utils_files/src/window_state.rs: new(): Storage object not found; \n\t"
                        .to_string(),
                ),
            },
            Err(error) => Err(format!(
                "utils_files/src/window_state.rs: new(): Storage object not found; \n\t{:?}",
                error
            )),
        }
    }

    fn get_storage_item(local_storage: &Storage, item: &str) -> Option<String> {
        match local_storage.get_item(item) {
            Ok(value) => value,
            Err(_) => None,
        }
    }

    fn get_day_state(local_storage: &Storage) -> Result<bool, String> {
        match local_storage.get_item("day_setting") {
            Ok(value) => match value
                .unwrap_or_else(|| {
                    local_storage.set_item("day_setting", "day").unwrap();
                    "day".to_string()
                })
                .as_str()
            {
                "day" => Ok(true),
                _ => Ok(false),
            },
            Err(error) => Err(format!(
                "utils_files/src/window_state.rs: new(): Could not access storage; \n\t{:?}",
                &error
            )),
        }
    }

    pub fn clear_storage(&self) {
        self.local_storage.clear().unwrap();
    }
}
