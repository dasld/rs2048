use std::ops/*::{Add, Sub}*/;
use std::fmt/*::Display*/;


struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "<<<{}, {}>>>", self.x, self.y)
    }
}


fn main() {
    let p = Point::origin() + Point { x: -14, y: 17 };
    //println!("Hello: {},{}.", p.x, p.y);
    println!("Hello: {}.", p);
}
