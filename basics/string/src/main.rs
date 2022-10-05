fn main() {
    let data = "initial value";
    let mut s = data.to_string();
    s.push_str(", added value");
    println!("modified s = {}", s);

    let hello = String::from("hello, ");
    // let hello = "hello";
    // let world = String::from("world!");
    let world = "world!";
    let hello_world = hello + world; // hello can't be used anymore (no &)
    // let hello_world = hello + world; /// cant concatednate a &str with ..
    println!("hello_world = {}", hello_world);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s123 = format!("{}-{}-{}", s1, s2, s3);
    println!("s123 = {}", s123);

    for c in s123.chars() {
        println!("{}", c);
    }

    for b in s123.bytes() {
        println!("{}", b);
    }
}
