use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please guess a number！");
    let secret_number = rand::thread_rng().gen_range(1..101); // 1..=100
    println!("The secret number: {}", secret_number);
    
    
    
    loop {
        println!("Please enter a number:");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess) // 這邊不能有 ;
            .expect("Failed to read line.");
            
        // let guess: u32 = guess.trim().parse().expect("Please enter a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    
    
    
}
