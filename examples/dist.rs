#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::time::Instant;

use service_levels::fn_wrapper;

#[derive(Copy, Clone)]
struct Point(f32, f32);

fn dist_euclidean(p1: Point, p2: Point) -> f32 {
    ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).sqrt()
}

fn dist_manhattan(p1: Point, p2: Point) -> f32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn_wrapper!(Dist, (Point, Point) => f32, 0: dist_euclidean, 1: dist_manhattan);

fn main() {
    let p1 = Point(0.0, 0.0);
    let p2 = Point(1.0, 1.0);
    let start = Instant::now();
    dbg!(Dist::<0>(p1, p2));
    println!("Level 0 took {:?}", start.elapsed());
    let start = Instant::now();
    dbg!(Dist::<1>(p1, p2));
    println!("Level 1 took {:?}", start.elapsed());
}
