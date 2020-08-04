use rand::prelude::*;
use crate::point::Point;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Cell {
    pub is_wall: bool
}

impl Cell {
    pub fn new(is_wall: bool) -> Cell{
        return Cell{is_wall:is_wall};
    }
}


#[derive(Clone, Copy)]
enum WallBlock {
    Forward,
    Left,
    Right
}

struct Direction {
    is_x: bool,
    is_neg: bool
}

impl Direction {
    fn new(is_x: bool, is_neg: bool) -> Direction {
        return Direction {
            is_x: is_x,
            is_neg: is_neg
        };
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    pub height: usize,
    pub width: usize
}

impl Grid {
    pub fn new(shape: Point) -> Result<Grid, &'static str> {
        // TODO create walls
        if shape.x < 1 {
            return Err("Width must be at least 1");
        }
        if shape.y < 1 {
            return Err("Height must be at least 1");
        }
        let directions = vec![
            Direction::new(false, false),
            Direction::new(true, false),
            Direction::new(false, true),
            Direction::new(true, true)
        ];
        let size = shape.x * shape.y;
        let mut cells = vec![Cell { is_wall: false}; size];
        let number_of_walls = 30;
        let mut rng = rand::thread_rng();
        for _ in 0..number_of_walls {
            let wall = Grid::build_wall(&mut rng);
            let mut cell_index = rng.gen_range(0, size);
            let mut x = cell_index % shape.x;
            let mut y = cell_index / shape.x;
            let mut direction_index = rng.gen_range(0, 4);
            for block in wall {
                direction_index = match block {
                    WallBlock::Forward => direction_index,
                    WallBlock::Left => {
                        if direction_index >= 3 { 0 } else { direction_index+1 }
                    },
                    WallBlock::Right => {
                        if direction_index <= 0 { 3 } else { direction_index-1 }
                    },
                };
                let direction = &directions[direction_index];
                
                if direction.is_x {
                    x = Grid::offset(x, direction.is_neg, shape.x - 1);
                } else {
                    y = Grid::offset(y, direction.is_neg, shape.y - 1);
                }
                cells[cell_index].is_wall = true;
                cell_index = y * shape.x + x;
            }

        }
        
        return Ok(Grid {
            cells: cells,
            height: shape.x,
            width: shape.y
        });
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        let index = self.width*y + x;
        if index >= self.width*self.height {
            return None;
        }
        return Some(self.cells[index])
    }

    fn offset(value: usize, subtract: bool, max: usize) -> usize {
        if subtract {
            if value < 1 {
                return max;
            }
            return value - 1;
        }
        if value >= max {
            return 0;
        }
        return value + 1;

    }

    fn build_wall(rng: &mut ThreadRng) -> Vec<WallBlock> {
        let length = rng.gen_range(15, 41);
        let mut wall = Vec::with_capacity(length);
        for i in 0..length {
            let turn = rng.gen_bool(0.1);
            let block: WallBlock;
            if turn {
                block = if rng.gen_bool(0.5) { WallBlock::Left } else { WallBlock::Right };
            } else {
                block = WallBlock::Forward
            }
            wall.push(block);
        }
        return wall;
    }
}