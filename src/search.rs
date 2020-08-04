use crate::point::Point;

pub trait Search {
    fn step(&mut self) -> bool;
    fn get_cell_state(&self, position: Point) -> Option<State>;
}

#[derive(Clone, Copy)]
pub enum State {
    Visited,
    Current,
    ChosenPath,
    NotVisited
}