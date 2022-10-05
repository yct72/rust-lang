fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    // error - v.push(6);
    println!("third: {}", third);   
    v.push(6);
    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("the third element doesn't exist"),
    }

    for i in &v {
        println!("{}", i);
    }
  
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
  
}
