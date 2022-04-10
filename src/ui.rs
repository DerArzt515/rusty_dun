use crate::models::Tile;
use crate::models::Frame;

pub struct Thing {
    pub a: usize,
}

fn get_tile_sym(tile: &Tile) -> char {
    match tile  {
        HALL => '.',
        ROOM => '#',
        _ => ' '
    }
}

pub fn draw_frame(frame: Frame<Tile>) {
    print!("\n");
    for row in frame.rows(){
        for cell in row {
            print!("{}", get_tile_sym(cell))
        }
        print!("\n");
    }
}
