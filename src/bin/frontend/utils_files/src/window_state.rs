use web_sys::Storage;
use web_sys::Window;

use crate::web_error::ClientError;

#[derive(PartialEq)]
pub struct ClientWindow {
    pub window: web_sys::Window,
    pub local_storage: web_sys::Storage,
    pub player_id_tag: Option<String>,
    pub player_id_key: Option<String>,
    pub settings: Option<String>,
    pub day: bool,
}

impl ClientWindow {
    pub fn new() -> Result<Self, ClientError> {
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

    fn get_window() -> Result<Window, ClientError> {
        web_sys::window().ok_or(ClientError::from(file!(), "new(): Window object not found"))
    }

    fn get_local_storage(window: &Window) -> Result<Storage, ClientError> {
        window
            .local_storage()
            .map_err(|error: _| {
                ClientError::from(
                    file!(),
                    &format!("new(): Storage object not found: {:?}", error),
                )
            })?
            .ok_or(ClientError::from(
                file!(),
                "new(): Storage object not found",
            ))
    }

    fn get_storage_item(local_storage: &Storage, item: &str) -> Option<String> {
        local_storage.get_item(item).ok().flatten()
    }

    fn get_day_state(local_storage: &Storage) -> Result<bool, ClientError> {
        match local_storage
            .get_item("day_setting")
            .map_err(|error: _| {
                ClientError::from(
                    file!(),
                    &format!("new(): Could not access storage: {:?}", error),
                )
            })?
            .unwrap_or_else(|| {
                local_storage.set_item("day_setting", "day").unwrap();
                "day".to_string()
            })
            .as_str()
        {
            "day" => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn clear_storage(&self) {
        self.local_storage.clear().unwrap();
    }
}
