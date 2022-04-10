mod setting;
mod fileio;
mod dungeon;
mod models;

const SETTINGS_LOCATION: &str = "./res/settings.toml";



fn main() {
    let settings: setting::Settings = setting::load_settings(SETTINGS_LOCATION);
    dungeon::create_floor(settings);
}
