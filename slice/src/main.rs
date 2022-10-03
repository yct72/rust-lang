fn main() {
    // word and s
    let mut s1 = String::from("hello world");
    let word = first_word(&s1);
    println!("word = {}", word);
    s1.clear();
    println!("word after s is clear = {}", word);

    // slice
    let s2 = String::from("hello world");

    let hello = &s2[..5];
    let world = &s2[6..];

    println!("hello = \"{}\"", hello);
    println!("world = \"{}\"", world);

    // first word slice
    let s3 = String::from("hello world");
    let word = first_word_slice(&s3);
    
    // error because we have immutable 'word' used later
    // s3.clear();
    
    println!("first word slice = {}", word);

    // array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str { // &str is string literal, slice
    // better: fn first_word_slice(s: &str) -> &str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}