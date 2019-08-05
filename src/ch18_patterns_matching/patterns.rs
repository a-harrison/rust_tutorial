struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // -- Destructuring structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // -- Simplified Destructuring
    let p2 = Point { x: 0, y: 7 };

    let Point { x, y } = p2;
    assert_eq!(0, x);
    assert_eq!(7, y);

}
