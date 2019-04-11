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
}
