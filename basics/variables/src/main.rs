fn main() {
    // variables
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    // constants
    const A_CONSTANT: u32 = 60 * 60 * 3;
    println!("A_CONSTANT: {}", A_CONSTANT);

    // shadowing
    let y = 1;
    println!("Initialize y: {}", y);
    let y = y + 1;
    println!("y = y + 1 = {}", y);

    {
        let y = y * 2;
        println!("y = y * 2 = {}", y);
    }

    println!("End shadowing, y = {}", y);

    // change type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces from string to number, len: {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("Mutable variables' type can't be change!");
    
}
