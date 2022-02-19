enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Loonie,
    Toonie,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Loonie => 100,
        Coin::Toonie => 200,
    }
}

fn main() {
    
    println!("Hello, cents! a dime is {} cents", value_in_cents(Coin::Dime));
}
