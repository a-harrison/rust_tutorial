pub fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    let x2 = vec![1, 2, 3];
    let equal_to_x2 = move |z| z == x2;
    // println!("Can't use x2 here: {:?}", x2);

    let y = vec![1,2,3];
    assert!(equal_to_x2(y));
}
