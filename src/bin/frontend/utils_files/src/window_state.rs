use crate::web_error::ClientError;
use web_sys::Storage;
use web_sys::Window;

trait SettingsTrait {
    fn get_day_state(&self) -> bool;
    fn get_animation_level_state(&self) -> u8;
}

impl SettingsTrait for String {
    fn get_day_state(&self) -> bool {
        match self.as_str() {
            "day" => true,
            _ => false,
        }
    }

    fn get_animation_level_state(&self) -> u8 {
        match self.as_str() {
            "High" => 2,
            "Low" => 1,
            _ => 0,
        }
    }
}

#[derive(PartialEq)]
pub struct ClientWindow {
    pub window: web_sys::Window,
    pub local_storage: web_sys::Storage,
    pub player_id_tag: Option<String>,
    pub player_id_key: Option<String>,
    pub settings: Option<String>,
    pub animation_level: u8,
    pub day: bool,
}

impl ClientWindow {
    pub fn new() -> Result<Self, ClientError> {
        let window: Window = Self::get_window()?;
        let local_storage: Storage = Self::get_local_storage(&window)?;
        let player_id_tag: Option<String> = Self::get_storage_item(&local_storage, "player_id_tag");
        let player_id_key: Option<String> = Self::get_storage_item(&local_storage, "player_id_key");
        let settings: Option<String> = Self::get_storage_item(&local_storage, "player_settings");
        let animation_level: u8 =
            Self::get_stored_storage_item(&local_storage, "player_animation_level", "high")?
                .get_animation_level_state();
        let day: bool =
            Self::get_stored_storage_item(&local_storage, "day_setting", "day")?.get_day_state();
        Ok(Self {
            window,
            local_storage,
            player_id_tag,
            player_id_key,
            settings,
            animation_level,
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

    fn get_stored_storage_item(
        local_storage: &Storage,
        item: &str,
        default: &str,
    ) -> Result<String, ClientError> {
        Ok(local_storage
            .get_item(item)
            .map_err(|error: _| {
                ClientError::from(
                    file!(),
                    &format!("new(): Could not access storage: {:?}", error),
                )
            })?
            .unwrap_or_else(|| {
                local_storage.set_item(item, default).unwrap();
                default.to_string()
            }))
    }

    pub fn set_player_id_tag(&mut self, new_player_id_tag: String) {
        self.player_id_tag = Some(new_player_id_tag);
    }

    pub fn clear_storage(&self) {
        self.local_storage.clear().unwrap();
    }
}
