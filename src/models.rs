pub struct Frame<T> {
    data: Vec<Vec<T>>,
}

impl<T> Frame<T> {
    pub fn get(&self, row: usize, col: usize) -> &T {
        if self.location_in_frame(row, col) {
            return &self.data[row][col];
        }
        panic!("Attempted to access location row = {} col = {} when frame has dimensions of rows = {} cols = {}", row, col, self.get_num_rows(), self.get_num_cols());
    }

    pub fn get_num_rows(&self) -> usize {
        return self.data.len();
    }

    pub fn get_num_cols(&self) -> usize {
        return self.data[0].len();
    }

    pub fn rows(&self) -> &Vec<Vec<T>> {
        return &self.data;
    }

    pub fn location_in_frame(&self, row: usize, col: usize) -> bool {
        return self.get_num_rows() > row && self.get_num_cols() > col; 
    }

    pub fn new(height: usize, width: usize, cell_generator: fn() -> T) -> Frame<T> {
        let mut data: Vec<Vec<T>> = Vec::new();

        for r in 0usize..height {
            let mut row = Vec::new();
            for c in 0usize..width {
                row.push(cell_generator());
            }
            data.push(row)
        }
        return Frame { data: data };
    }
}

pub enum Tile {
    WALL { hardness: u16 },
    HALL{ hardness: u16 },
    ROOM{ hardness: u16 },
}

pub fn make_wall(max_hardness: u16, gen: &dyn Fn() -> u16 ) -> Tile {
    Tile::WALL{hardness: gen()}
}

pub fn make_hall() -> Tile {
    Tile::HALL{hardness: 0}
}


pub struct Dungeon {
    pub map: Frame<Tile>,
    pub height: usize,
    pub width: usize,
}
