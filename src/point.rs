use std::hash::{Hash, Hasher};
use std::cmp::max;
use std::ops::Sub;
use std::cmp::Ordering;
use rand::Rng;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point
{
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        return Point{ x:x, y:y };
    }

    pub fn rand(max_x: usize, max_y: usize) -> Point {
        let mut rng = rand::thread_rng();
        return Point {
            x: rng.gen_range(0, max_x),
            y: rng.gen_range(0, max_y)
        }
    }

    pub fn distance_to(&self, other: Self) -> f64 {        
        let x_diff = _abs_diff(self.x, other.x) as f64;
        let y_diff = _abs_diff(self.y, other.y) as f64;
        return (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt();
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x {
            return Some(self.x.cmp(&other.x));
        }
        if self.y == other.y {
            return Some(self.y.cmp(&other.y));
        }
        return None;
    }
}


impl Hash for Point {    
    fn hash<H: Hasher>(&self, hasher: &mut H){ 
        self.x.hash(hasher);
        self.y.hash(hasher);
    }
}

impl Eq for Point {

}


fn _abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}