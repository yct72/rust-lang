fn how_long(value: i32, unit: char) {
    println!("{}{} long!", value, unit);
}
fn expression() -> i32 { // '->'
    let x = 3;
    x + 1 // no ';'
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    /*function */
    how_long(5, 'm');
    /* expression */
    println!("expression is {}", expression());
    println!("five() is {}", five());
    let x = 1;
    println!("x plus one = {}", plus_one(x));
}
