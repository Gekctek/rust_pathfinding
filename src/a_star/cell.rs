use std::hash::Hasher;
use std::hash::Hash;
use crate::point::Point;
use std::cmp::Ordering;

pub struct AStarCell {
    pub position: Point,
    //Start -> cell
    pub g: f64,
    //Estimate cell -> end
    pub h: f64
}

impl AStarCell {
    pub fn get_f(&self) -> f64 {
        return self.g + self.h;
    }
}

impl Ord for AStarCell {    
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => Ordering::Equal
        }
    }
}

impl PartialOrd for AStarCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_f().partial_cmp(&other.get_f()) {
            Some(ordering) => Some(ordering.reverse()),
            None => None
        }
    }
}

impl Eq for AStarCell {

}

impl PartialEq for AStarCell {
    fn eq(&self, other: &Self) -> bool {
        return self.get_f() == other.get_f();
    }
}

impl Hash for AStarCell {    
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.position.hash(hasher);
    }
}
