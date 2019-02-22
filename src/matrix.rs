use std::fmt;

#[derive(Debug)]
pub struct Matrix {
    pub data: Vec<u64>,
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        let mut start_index = 0;
        let mut end_index = self.col;
        while end_index <= self.data.len() {
            write!(f, "{:?}\n", &self.data[start_index..end_index])?;
            start_index = end_index;
            end_index += self.col;
        }
        write!(f, "({} x {})", &self.row, &self.col)
    }
}

impl Matrix {
    pub fn new(data: Vec<u64>, row: usize, col: usize) -> Matrix {
        if (col * row) == data.len() {
            Matrix { data, row, col }
        } else {
            panic!(
                "{} not enough elements to populate {} x {} matrix.",
                data.len(),
                row,
                col
            );
        }
    }

    pub fn to_position(&self, row_coord: usize, col_coord: usize) -> usize {
        (row_coord * self.col) + (col_coord)
        // row_coord.saturating_sub(1) * self.col + col_coord
    }

    // fn to_coords(&self, position: usize) -> (usize, usize) {
    //     ( position % self.col, position.saturating_add(self.col).saturating_add(1) )
    // }
}
