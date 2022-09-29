use std::io;

fn main() {
    let mut x = String::new();
    println!("Please enter x:");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read!");
    let x:i32 = x.trim().parse().expect("Please enter a number!");
    if x > 1 {
        println!("x > 1");
    } else {
        println!("x <= 1");
    }

    /* bool */
    let y = 3;
    // fail:
    // if y {
    //     println!("y is 3!")
    // }
    if y == 3 {
        println!("y is 3!")
    }
    
    /* use 'if' in 'let' */
    let condition = true;
    let z = if condition { 1 } else { 0 };
    println!("z = {}", z);
}   
