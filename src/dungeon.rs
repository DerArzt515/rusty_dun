use crate::models::make_hall;
use crate::models::Frame;
use crate::setting::Settings;
use crate::models::Dungeon;

pub fn create_floor(settings: Settings) -> Dungeon {
    println!("creating floor");
    let tile_gen = || make_hall();
    return Dungeon {
        map: Frame::new(
            settings.dungeon.height,
            settings.dungeon.width,
            tile_gen,
        ),
        height: settings.dungeon.height,
        width: settings.dungeon.width,
    };
}
