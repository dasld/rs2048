//use std::ops/*::{Add, Sub}*/;
use std::fmt;
use crate::utils::NegativeIntegerError;


// this is the X or Y value of a Point
pub type Coord = i32;
pub type PointResult = Result<Point, NegativeIntegerError>;


pub struct Point {
    x: Coord,
    y: Coord,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn new(x: Coord, y: Coord) -> PointResult {
	if x < 0 || y < 0 {
	    Err(NegativeIntegerError)
	} else {
            Ok(Point { x: x, y: y })
	}
    }

    pub fn translate(&mut self, x: Coord, y: Coord) -> Result<(), NegativeIntegerError> {
	let x2 = self.x + x;
        let y2 = self.y + y;
	if x2 < 0 || y2 < 0 {
	    println!("{} cannot translate to negative coordinates.", &self);
	    Err(NegativeIntegerError)
	} else {
	    self.x = x2;
	    self.y = y2;
	    Ok(())
	}
    }
}

/*
impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}
*/

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `f` implements the `Write` trait, which is what the
        // write! macro expects. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "Point({},{})", self.x, self.y)
    }
}
