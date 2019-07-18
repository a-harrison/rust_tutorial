fn main() {
    let some_u8_value = Some(0u8);

    // Standard
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Simpler, non-exhaustive
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // Another example
    // Match State Quarters or count coins 
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("(State quarter from {:?})", state),
        _ => count += 1,
    }

    // Simpler version
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("(State quarter from {:?})", state);
    } else {
        count += 1;
    }
}
