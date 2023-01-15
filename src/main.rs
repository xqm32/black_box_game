use std::fmt::Display;

use rand::{rngs::ThreadRng, seq::index::sample, thread_rng};

#[derive(Debug, Clone)]
enum Grid {
    Empty,
    Ball,
    Ray,
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

    fn get(&self, row: usize, col: usize) -> &Grid {
        // TODO: 添加越界检查
        &self.grid[row * self.cols + col]
    }
}

impl Display for BlackBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?}\t", self.get(i, j))?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let black_box = BlackBox::new(4, 4, 4);
    println!("{}", black_box);
}
