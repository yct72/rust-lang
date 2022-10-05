fn main() {
    // match form
    let config_max = Some(3u8);
    // let config_max: Option<i32> = None;
    match config_max {
        Some(max) => println!("max is {}", max),
        _ => (),
    }

    // if let form
    if let Some(max) = config_max {
        println!("max is {}", max);
    } else {
        println!("else!");
    }
}
