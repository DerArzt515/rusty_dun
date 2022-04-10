use crate::models::Dungeon;
use crate::models::Tile;
use crate::models::Frame;

mod ui;
mod setting;
mod fileio;
mod dungeon;
mod models;

const SETTINGS_LOCATION: &str = "./res/settings.toml";



fn main() {
    let settings: setting::Settings = setting::load_settings(SETTINGS_LOCATION);
    
    println!("{:?}", settings);
    let dungeon: Dungeon = dungeon::create_floor(settings);
    ui::draw_frame(dungeon.map);
}
