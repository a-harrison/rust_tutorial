fn main() {
    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        California
        // snip
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("(State quarter from {:?})", state);
                25
            },
        }
    }

    let coin = Coin::Penny;
    let quarter = Coin::Quarter(UsState::California);
    println!("The value of the coin is {}.", value_in_cents(coin));
    println!("The value of the other coin is {}.", value_in_cents(quarter));

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Placeholder example
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
