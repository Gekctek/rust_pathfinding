use std::collections::HashSet;
use crate::search::{Search,State};
use std::collections::{BinaryHeap, HashMap};
use crate::grid::{Grid,Cell};
use crate::point::Point;
use crate::a_star::cell::AStarCell;
use std::result::Result;

pub struct AStarSearch {
    grid: Grid,
    start: Point,
    end: Point,
    open_set: BinaryHeap<AStarCell>,
    were_open_set: HashSet<Point>,
    closed_set: HashMap<Point, AStarCell>,
    path_found: bool
}

impl AStarSearch {
    pub fn create(start: Point, end: Point, grid: Grid) -> Result<AStarSearch, String> {
        if let None = grid.get_cell(start.x, start.y) {
            return Err(format!("Start cell (x:{},y:{}) does not exist on the grid.", start.x, start.y));
        }
        if let None = grid.get_cell(end.x, end.y) {
            return Err(format!("End cell (x:{},y:{}) does not exist on the grid.", end.x, end.y));
        }
        let start_cell = AStarCell { g: 0.0, h: f64::MAX, position: start };
        let mut open_set = BinaryHeap::new();
        open_set.push(start_cell);
        let mut were_open_set = HashSet::new();
        were_open_set.insert(start);
        let closed_set = HashMap::new();
        return Ok(AStarSearch {
            grid: grid,
            start: start,
            end: end,
            open_set: open_set,
            were_open_set: were_open_set,
            closed_set: closed_set,
            path_found: false
        });
    }
}

impl Search for AStarSearch {
    fn step(&mut self) -> bool {
        if self.path_found {
            return true;
        }
        match self.open_set.pop() {
            Some(next_cell) => {
                let cell_position = next_cell.position;
                self.closed_set.insert(next_cell.position, next_cell);
                let is_end = cell_position == self.end;
                if is_end {
                    self.path_found = true;
                    return true;
                }
                for x_offset in 0..3 {
                    for y_offset in 0..3 {
                        if x_offset == 1 && y_offset == 1 {
                            continue;
                        }
                        
                        let x = cell_position.x + x_offset;
                        let y = cell_position.y + y_offset;
                        let out_of_bounds = x < 1 || y < 1 || x >= self.grid.width || y >= self.grid.height;
                        if out_of_bounds {
                            //Skip because need to offset by one but using usize
                            continue;
                        }
                        let x = x-1;
                        let y = y-1;
                        let position = Point::new(x, y);
    
                        let g = 1.0; // Always same difficulty
                        let h = position.distance_to(self.end);
                        // let h = position.distance_to(self.start);
                        let successor_cell = match self.grid.get_cell(x, y){
                            Some(Cell{is_wall: false}) => AStarCell { position: position, g: g, h: h },
                            _ => continue // Wall or out of bounds
                        };
                        let inserted = self.were_open_set.insert(successor_cell.position);
                        if inserted {                            
                            self.open_set.push(successor_cell);
                        }
                    }
                }
            },
            None => {
                // No path to the end, use best path
                self.path_found = true;
                return true;
            }
        }
        return false;
    }
    
    fn get_cell_state(&self, position: Point) -> Option<State> { 
        if let Some(cell) = self.grid.get_cell(position.x, position.y){
            if cell.is_wall {
                return None;
            }
            
            if self.closed_set.contains_key(&position) {
                return Some(if self.path_found {State::ChosenPath} else {State::Visited});
            }
            for open in self.open_set.iter() {
                if open.position == position {
                    return Some(State::Current);
                }
            }
            return Some(State::NotVisited);
        }
        return None;
    }
}
