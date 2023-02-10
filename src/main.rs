use std::{fmt::Display, io};

use rand::{rngs::ThreadRng, seq::index::sample, thread_rng};

#[derive(Debug, Clone)]
enum Grid {
    Empty,
    Ball,
    Ray,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grid::Empty => write!(f, "-")?,
            Grid::Ball => write!(f, "O")?,
            Grid::Ray => write!(f, "*")?,
        }
        Ok(())
    }
}

#[derive(Debug)]
struct BlackBox {
    rows: usize,
    cols: usize,
    grid: Vec<Grid>,
    rng: ThreadRng,
}

impl BlackBox {
    fn new(rows: usize, cols: usize, amount: usize) -> BlackBox {
        let mut black_box = BlackBox {
            rows: rows,
            cols: cols,
            grid: Vec::with_capacity(rows * cols),
            rng: thread_rng(),
        };
        black_box.grid.resize(rows * cols, Grid::Empty);
        black_box.gen_balls(amount);
        black_box
    }

    fn gen_balls(&mut self, amount: usize) {
        for i in sample(&mut self.rng, &self.rows * &self.cols, amount).iter() {
            self.grid[i] = Grid::Ball;
        }
    }

    fn get(&self, row: isize, col: isize) -> Option<&Grid> {
        if row < 0 || row >= (self.rows as isize) || col < 0 || col >= (self.cols as isize) {
            return None;
        }
        Some(&self.grid[(row * (self.cols as isize) + col) as usize])
    }

    fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut Grid> {
        if row < 0 || row >= (self.rows as isize) || col < 0 || col >= (self.cols as isize) {
            return None;
        }
        Some(&mut self.grid[(row * (self.cols as isize) + col) as usize])
    }

    fn shoot(&mut self, row: isize, col: isize, drow: isize, dcol: isize) -> String {
        let mut row = row;
        let mut col = col;
        let mut drow = drow;
        let mut dcol = dcol;
        loop {
            if let Some(grid) = self.get(row, col) {
                match grid {
                    Grid::Ball => return format!("Hit"),
                    _ => (),
                }
            } else {
                return format!("Wall at ({}, {})", row, col);
            }
            *self.get_mut(row, col).unwrap() = Grid::Ray;

            if let Some(grid) = self.get(row + drow, col + dcol) {
                match grid {
                    Grid::Ball => return format!("Hit"),
                    _ => (),
                }
            }

            let mut forward_balls = 0;
            if drow != 0 {
                if let Grid::Ball = self.get(row + drow, col + drow).unwrap_or(&Grid::Empty) {
                    forward_balls += 1;
                    (drow, dcol) = (dcol, -drow);
                }
                if let Grid::Ball = self.get(row + drow, col - drow).unwrap_or(&Grid::Empty) {
                    forward_balls += 1;
                    (drow, dcol) = (dcol, drow)
                }
            } else if dcol != 0 {
                if let Grid::Ball = self.get(row + dcol, col + dcol).unwrap_or(&Grid::Empty) {
                    forward_balls += 1;
                    (drow, dcol) = (-dcol, drow);
                }
                if let Grid::Ball = self.get(row - dcol, col + dcol).unwrap_or(&Grid::Empty) {
                    forward_balls += 1;
                    (drow, dcol) = (dcol, drow)
                }
            }
            if forward_balls == 2 {
                return format!("Reflection");
            }

            row += drow;
            col += dcol;
        }
    }

    fn clear_ray(&mut self) {
        for i in &mut self.grid {
            if let Grid::Ray = i {
                *i = Grid::Empty;
            }
        }
    }
}

impl Display for BlackBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                // 此处不会越界，因此不使用 get()
                write!(f, "{} ", self.grid[i * self.cols + j])?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut black_box = BlackBox::new(16, 16, 16);
    println!("{}", black_box);

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Error");

        let (dir, loc) = match buf.split_once(" ") {
            Some((dir, loc)) => (dir, loc),
            None => {
                println!("Error input");
                continue;
            }
        };

        let loc = match loc.trim().parse::<isize>() {
            Ok(i) => i,
            Err(_) => {
                println!("Error location, buf: {}", buf);
                continue;
            }
        };

        match dir.trim() {
            "left" | "l" => println!("{}", black_box.shoot(loc, 0, 0, 1)),
            "right" | "r" => println!(
                "{}",
                black_box.shoot(loc, (black_box.cols - 1) as isize, 0, -1)
            ),
            "top" | "t" => println!("{}", black_box.shoot(0, loc, 1, 0)),
            "bottom" | "b" => println!(
                "{}",
                black_box.shoot((black_box.rows - 1) as isize, loc, -1, 0)
            ),
            _ => {
                println!("Error direction");
                continue;
            }
        }

        println!("{}", black_box);
        black_box.clear_ray();
    }
}
