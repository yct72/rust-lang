enum Coin {
    Penny, 
    Nickel, 
    Dime,
    Quarter,
}


fn main() {
    let one_penny = Coin::Penny;
    println!("One penny is {} cent!", value_in_cents(one_penny));

    let one_quarter = Coin::Quarter;
    println!("One quarter are {} cents!", value_in_cents(one_quarter));

    // match Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    assert_eq!(six, Some(6));
    assert_eq!(none, None);

    // other
    is_101(5);
    is_101(101);
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("You got a quarter!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn is_101(x: i32) {
    match x {
        101 => {
            println!("is101: Yes!");
        },
        other => {
            println!("is101: No! It is {}!", other);
        },
        // or _ => () when not using the value
    }
}
