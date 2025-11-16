use std::collections::HashSet;

struct Field {
    cells: Vec<Vec<u8>>,
    shown: Vec<Vec<bool>>,
    width: u8,
    height: u8,
    mines: u16,
}

impl Field {
    fn place_mines(width: u8, height: u8, mines: u16) -> HashSet<(u8, u8)> {
        let mut positions = HashSet::new();
        let mut counter = mines;

        while counter > 0 {
            let x = rand::random_range(0..width);
            let y = rand::random_range(0..height);
            if positions.contains(&(x, y)) {
                continue;
            } else {
                positions.insert((x, y));
                counter -= 1;
            }
        }

        positions
    }

    fn generate_field(width: u8, height: u8, positions: HashSet<(u8, u8)>) -> Vec<Vec<u8>> {
        let mut field = Vec::new();
        for _ in 0..height {
            field.push(Vec::with_capacity(width as usize));
        }

        for (x, y) in positions {
            field[y as usize][x as usize] = 7;
        }

        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                let mut mines = 0;
                if let Some(cell) = field.get_mut(y) {
                    if let Some(7) = cell.get_mut(x) {
                        continue;
                    }
                    if let Some(7) = cell.get_mut(x - 1) {
                        mines += 1;
                    }
                    if let Some(7) = cell.get_mut(x + 1) {
                        mines += 1;
                    }
                }
                if let Some(cell) = field.get_mut(y - 1) {
                    if let Some(7) = cell.get_mut(x) {
                        mines += 1;
                    }
                    if let Some(7) = cell.get_mut(x + 1) {
                        mines += 1;
                    }
                }
                if let Some(cell) = field.get_mut(y + 1) {
                    if let Some(7) = cell.get_mut(x) {
                        mines += 1;
                    }
                    if let Some(7) = cell.get_mut(x + 1) {
                        mines += 1;
                    }
                }

                field[y as usize][x as usize] = mines;
            }
        }

        field
    }

    fn zero_filled(width: u8, height: u8) -> Vec<Vec<u8>> {
        let mut result = Vec::new();

        for _ in 0..height {
            result.push(vec![0; width as usize]);
        }

        result
    }

    pub fn new(width: u8, height: u8, mines: u16) -> Result<Self, ()> {
        if height % 2 == 0 {
            return Err(());
        }
        if mines > width as u16 * height as u16 {
            return Err(());
        }

        Ok(Self {
            cells: Self::generate_field(width, height, Self::place_mines(width, height, mines)),
            shown: Vec::new(),
            width,
            height,
            mines,
        })
    }

    pub fn reveal(&mut self, x: usize, y: usize, show: bool) {
        if self.cells[y][x] == 7 {
            if show {
                // game over
            }
        } else {
            self.shown[y][x] = true;
            if self.cells[y][x] == 0 {
                if x > 0 {
                    self.reveal(x - 1, y, false);
                    if y > 0 {
                        self.reveal(x - 1, y - 1, false);
                    }
                }
                if x < self.width as usize {
                    self.reveal(x + 1, y, false);
                    if y > 0 {
                        self.reveal(x + 1, y - 1, false);
                    }
                }
                if y > 0 {
                    self.reveal(x, y - 1, false);
                }
                if y < self.height as usize {
                    self.reveal(x, y + 1, false);
                }
            }
        }
    }
}