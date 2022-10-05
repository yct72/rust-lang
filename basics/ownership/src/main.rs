fn main() {
    let s1 = String::from("I'm s1!");
    let mut s2 = String::from("I'm s2!");

    take_ownership(s1);

    // error
    // println!("s1 = {}", s1);

    s2 = take_ownership_and_give_back(s2);
    println!("s2 = {}", s2);

}

fn take_ownership(s: String) {
    println!("in take_ownership, s = {}", s);
}

fn take_ownership_and_give_back(s: String) -> String {
    println!("in take_ownership_and_give_back, s = {}", s);
    s
}

