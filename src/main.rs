
use std::{thread, time::Duration};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
}

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::BLACK; width * height],
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            Color::BLACK
        }
    }

    pub fn display(&self) {
        print!("[2J[1;1H");
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get_color(x, y);
                print!("{}", if c == Color::WHITE { "██" } else { "  " });
            }
            println!();
        }
    }
}

pub struct GameOfLife {
    grid: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>,
    framebuffer: Framebuffer,
    width: usize,
    height: usize,
    generation: usize,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let mut game = Self {
            grid: vec![vec![false; width]; height],
            next: vec![vec![false; width]; height],
            framebuffer: Framebuffer::new(width, height),
            width,
            height,
            generation: 0,
        };
        game.load_multiple_patterns();
        game
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let dirs = [-1, 0, 1];
        dirs.iter().flat_map(|&dy| {
            dirs.iter().filter_map(move |&dx| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let nx = ((x as isize + dx).rem_euclid(self.width as isize)) as usize;
                    let ny = ((y as isize + dy).rem_euclid(self.height as isize)) as usize;
                    Some(self.grid[ny][nx])
                }
            })
        }).filter(|&alive| alive).count()
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.grid[y][x];
                self.next[y][x] = match (alive, neighbors) {
                    (true, 2 | 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        std::mem::swap(&mut self.grid, &mut self.next);
        self.generation += 1;
    }

    pub fn render(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if self.grid[y][x] { Color::WHITE } else { Color::BLACK };
                self.framebuffer.point(x, y, color);
            }
        }
    }

    pub fn display(&self) {
        self.framebuffer.display();
    }

    pub fn load_multiple_patterns(&mut self) {
        self.grid = vec![vec![false; self.width]; self.height];

        let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (dx, dy) in &glider {
            self.grid[dy + 1][dx + 1] = true;
        }

        let blinker = vec![(0, 1), (1, 1), (2, 1)];
        for (dx, dy) in &blinker {
            self.grid[dy + 10][dx + 10] = true;
        }

        let toad = vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
        for (dx, dy) in &toad {
            self.grid[dy + 20][dx + 20] = true;
        }

        let beacon = vec![(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)];
        for (dx, dy) in &beacon {
            self.grid[dy + 30][dx + 5] = true;
        }

        let lwss = vec![
            (0, 0), (3, 0),
            (4, 1),
            (0, 2), (4, 2),
            (1, 3), (2, 3), (3, 3), (4, 3)
        ];
        for (dx, dy) in &lwss {
            self.grid[dy + 5][dx + 50] = true;
        }

        let custom = vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)];
        for (dx, dy) in &custom {
            self.grid[dy + 35][dx + 45] = true;
        }

        let pulsar = vec![
            (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
            (0, 2), (5, 2), (7, 2), (12, 2),
            (0, 3), (5, 3), (7, 3), (12, 3),
            (0, 4), (5, 4), (7, 4), (12, 4),
            (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
            (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
            (0, 8), (5, 8), (7, 8), (12, 8),
            (0, 9), (5, 9), (7, 9), (12, 9),
            (0, 10), (5, 10), (7, 10), (12, 10),
            (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
        ];
        for (dx, dy) in &pulsar {
            self.grid[dy + 2][dx + 65] = true;
        }

        let diehard = vec![
            (7, 0),
            (1, 1), (2, 1),
            (2, 2), (6, 2), (7, 2), (8, 2),
        ];
        for (dx, dy) in &diehard {
            self.grid[dy + 25][dx + 10] = true;
        }

        let acorn = vec![
            (1, 0),
            (3, 1),
            (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)
        ];
        for (dx, dy) in &acorn {
            self.grid[dy + 15][dx + 35] = true;
        }
    }
}

fn main() {
    let mut game = GameOfLife::new(80, 40);

    loop {
        game.render();
        game.display();
        game.update();
        thread::sleep(Duration::from_millis(100));
    }
}
