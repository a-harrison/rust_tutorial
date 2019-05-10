// Only accepts a single type T for both values
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

// Method can be used for any type T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Method is only defined for instances of Point<T> where T is f32.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ---------------------

// Supports different types for x and y values
#[allow(dead_code)]
struct MixedPoint<T, U> {
    x: T,
    y: U
}

// Somewhat hard to read, but creates a 3rd mixed point by combining values from 2 MixedPoints 
impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint <T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer.x = {}.", integer.x());
    println!("float.x = {}.", float.x());
    println!("");

    // let wont_work = Point { x: 5, y: 4.0 }; // won't work because of mixed types

    let integer_and_float = MixedPoint { x: 5, y: 10 };
    let integer_and_float = MixedPoint { x: 1.0, y: 2.0 };
    let integer_and_float = MixedPoint { x: 5, y: 4.0 };

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
