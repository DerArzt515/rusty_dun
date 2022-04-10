use crate::fileio::read_file_to_str;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Settings { 
    pub dungeon: DungeonSettings,
    pub room: RoomSettings,
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
    pub height: usize,
    pub width: usize,
    pub max_wall_hardness: usize,
    pub room_buffer: usize,
}

impl fmt::Debug for DungeonSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DungeonSettings")
            .field("height", &self.height)
            .field("width", &self.width)
            .field("max_wall_hardness", &self.max_wall_hardness)
            .field("room_buffer", &self.room_buffer)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct RoomSettings {
    pub min_width: usize,
    pub min_height: usize,
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

