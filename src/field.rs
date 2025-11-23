use std::collections::HashSet;

pub struct Field {
    cells: Vec<Vec<u8>>,
    shown: Vec<Vec<bool>>,
    width: u8,
    height: u8,
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
        let mut field = Self::filled(0, width, height);

        for (x, y) in positions {
            field[y as usize][x as usize] = 7;
        }

        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                let mut mines = 0;
                if x % 2 == 0 {
                    if let Some(cell) = field.get_mut(y) {
                        if let Some(7) = cell.get_mut(x) {
                            continue;
                        }
                        if x > 0 && let Some(7) = cell.get_mut(x - 1) {
                            mines += 1;
                        }
                        if x < width as usize && let Some(7) = cell.get_mut(x + 1) {
                            mines += 1;
                        }
                    }
                    if y < height as usize && let Some(cell) = field.get_mut(y + 1) {
                        if x > 0 && let Some(7) = cell.get_mut(x - 1) {
                            mines += 1;
                        }
                        if let Some(7) = cell.get_mut(x) {
                            mines += 1;
                        }
                        if x < width as usize && let Some(7) = cell.get_mut(x + 1) {
                            mines += 1;
                        }
                    }
                    if y > 0 && let Some(cell) = field.get_mut(y - 1) {
                        if let Some(7) = cell.get_mut(x) {
                            mines += 1;
                        }
                    }
                } else {
                    if let Some(cell) = field.get_mut(y) {
                        if let Some(7) = cell.get_mut(x) {
                            continue;
                        }
                        if x > 0 && let Some(7) = cell.get_mut(x - 1) {
                            mines += 1;
                        }
                        if x < width as usize && let Some(7) = cell.get_mut(x + 1) {
                            mines += 1;
                        }
                    }
                    if y > 0 && let Some(cell) = field.get_mut(y - 1) {
                        if x > 0 && let Some(7) = cell.get_mut(x - 1) {
                            mines += 1;
                        }
                        if let Some(7) = cell.get_mut(x) {
                            mines += 1;
                        }
                        if x < width as usize && let Some(7) = cell.get_mut(x + 1) {
                            mines += 1;
                        }
                    }
                    if y < height as usize && let Some(cell) = field.get_mut(y + 1) {
                        if let Some(7) = cell.get_mut(x) {
                            mines += 1;
                        }
                    }
                }

                field[y as usize][x as usize] = mines;
            }
        }

        field
    }

    fn filled<T: Copy + Clone>(value: T, width: u8, height: u8) -> Vec<Vec<T>> {
        let mut result = Vec::new();

        for _ in 0..height {
            result.push(vec![value; width as usize]);
        }

        result
    }

    pub fn new(width: u8, height: u8, mines: u16) -> Result<Self, ()> {
        // if height % 2 == 0 {
        //     return Err(());
        // }
        if mines > width as u16 * height as u16 {
            return Err(());
        }

        Ok(Self {
            cells: Self::generate_field(width, height, Self::place_mines(width, height, mines)),
            shown: Self::filled(false, width, height),
            width,
            height,
        })
    }

    pub fn reveal(&mut self, x: usize, y: usize, show: bool) {
        if self.shown[y][x] {
            return;
        }

        if self.cells[y][x] == 7 {
            if show {
                let (w, h) = self.size();
                for i in 0..h {
                    for j in 0..w {
                        if self.cells[i][j] == 7 {
                            self.shown[i][j] = true;
                        }
                    }
                }
            }
        } else {
            self.shown[y][x] = true;
            if self.cells[y][x] == 0 {
                if x % 2 == 0 {
                    if x > 0 {
                        self.reveal(x - 1, y, false);
                        if y < self.height as usize - 1 {
                            self.reveal(x - 1, y + 1, false);
                        }
                    }
                    if x < self.width as usize - 1 {
                        self.reveal(x + 1, y, false);
                        if y < self.height as usize - 1 {
                            self.reveal(x + 1, y + 1, false);
                        }
                    }
                    if y > 0 {
                        self.reveal(x, y - 1, false);
                    }
                    if y < self.height as usize - 1 {
                        self.reveal(x, y + 1, false);
                    }
                } else {
                    if x > 0 {
                        self.reveal(x - 1, y, false);
                        if y > 0 {
                            self.reveal(x - 1, y - 1, false);
                        }
                    }
                    if x < self.width as usize - 1 {
                        self.reveal(x + 1, y, false);
                        if y > 0 {
                            self.reveal(x + 1, y - 1, false);
                        }
                    }
                    if y > 0 {
                        self.reveal(x, y - 1, false);
                    }
                    if y < self.height as usize - 1 {
                        self.reveal(x, y + 1, false);
                    }
                }
            }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width as usize, self.height as usize)
    }

    pub fn get_cell(&self, x: usize, y: usize) -> (u8, bool) {
        (self.cells[y][x], self.shown[y][x])
    }
}