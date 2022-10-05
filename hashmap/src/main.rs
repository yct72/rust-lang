use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue team"), 10);
    scores.insert(String::from("yellow team"), 50);
    let team_name = String::from("blue team");
    println!("scores[\"blue team\"] = {:?}", scores.get(&team_name));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("-----------");
    
    // check whether the key exists
    scores.entry(String::from("green team")).or_insert(20);
    scores.entry(String::from("blue team")).or_insert(10);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    println!("-----------");
    // calculate count
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        println!("world = {}", word);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
