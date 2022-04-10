
pub struct Frame<T> {
    data: Vec<Vec<T>>,
}

impl<T> Frame<T> {
    fn get(&self, row: u32, col: u32) -> &T {
        return &self.data[row][col];
    }

    fn new(rows: u32, cols: u32, cell_generator: &dyn Fn()-> T) -> Frame<T> {
    let data: Vec<Vec<T>> = Vec::new();

    for r in 0u32..rows {
        let row = Vec::new();
        for c in 0u32..cols {
            row.push(cell_generator());
        }
        data.push(row)
    }
    return Frame{data: data};
}

