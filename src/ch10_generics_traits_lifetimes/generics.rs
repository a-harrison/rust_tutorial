// Previous version
// Since this is generic, it doesn't auto-implement the Copy trait
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// New implementation that requires a type that implements the PartialOrd Trait
// TODO: Implement this with clone instead of Copy (10.2 : 10-15)
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Return a reference to the largest T value in the slice, thereby avoiding heap allocations
fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_position = 0;

    for (i, item) in list.iter().enumerate() {
        if item > &list[largest_position] {
            largest_position = i;
        }
    }

    &list[largest_position]
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest number is {}.", result);

    let result = largest_reference(&char_list);
    println!("The largest char is {}.", result);
}
