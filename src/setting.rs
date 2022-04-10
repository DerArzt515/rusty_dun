use crate::fileio::read_file_to_str;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Settings { 
    dungeon: DungeonSettings,
    room: RoomSettings,
}

impl fmt::Debug for Settings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("")
            .field("dungeon", &self.dungeon)
            .field("room", &self.room)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct DungeonSettings {
    height: u16,
    width: u16,
}

impl fmt::Debug for DungeonSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DungeonSettings")
            .field("height", &self.height)
            .field("width", &self.width)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct RoomSettings {
    min_width: u16,
    min_height: u16,
}


impl fmt::Debug for RoomSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RoomSettings")
            .field("min_height", &self.min_height)
            .field("min_width", &self.min_width)
            .finish()
    }
}

pub fn load_settings(settings_location: &str) -> Settings {
    let settings = read_file_to_str(settings_location);
    return toml::from_str(&settings).unwrap();
}

