fn main() {
    let mut count = 0;
    // loop label
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("END!");

    // return value for loop
    let mut counter = 0;
    let ret = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("ret = {}", ret);

    // for loop
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element = {},", element);
    }

    println!("------rev------");

    for i in (1..4).rev() {
        println!("{}!", i);
    }
}
