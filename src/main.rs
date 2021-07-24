pub mod utils;
pub mod point;

use point::Point;


fn main() {
    let x = 14;
    let y = 17;
    /*
    match Point::new(x, y) {
	Ok(p) => println!("p is {}.", p),
	Err(e) => println!("{}", e),
	//let mut p = Point::new(-14, 17).unwrap();
	//p.translate(8, 5);
	//
    }
    */
    let mut p = Point::new(x, y).unwrap();
    p.translate(18, 18).unwrap();
    println!("p is {}.", p);
}
