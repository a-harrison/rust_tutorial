fn main() {
    // Generic form, need to declare type
    // let v: Vec<i32> = Vec::new();

    // Macro; type can be inferred by initial values
    // let v = vec![1, 2, 3];

    // Append to vector
    // let mut v = Vec::new();
    // v.push(5);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}.", third);

    // Accessing an element out of bounds returns None
    match v.get(9) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }

    // Iterating through a vector
    println!("Printing the contents of the vector:");
    for i in &v {
        println!("{}", i);
    }

    // Modifying elements of a vector
    println!("Printing the contents of the second vector:");
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("{}",i);
    }

    // Using an enum to store different values in a vector
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    println!("Printing the contents of the enum vector:");
    for i in &row {
        println!("{:?}", i);
    }
}
