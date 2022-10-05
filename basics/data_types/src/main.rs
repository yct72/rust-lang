use std::io;

fn main() {
    /* tuple */
    let tup = (500, 6.4, 1);
    // let tup: (i32, f32, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred = {}, six_point_four = {}, one = {}", 
        five_hundred, six_point_four, one);
    
    /* array */
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [1, 2, 3, 4, 5];
    println!("a[2] = {}", a[2]);

    println!("Please enter an index.");
    
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read.");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("The index is not a number.");
    
    let element = a[index];
    println!("a[{}] = {}", index, element);
}
