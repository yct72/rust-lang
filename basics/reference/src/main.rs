fn main() {
    // reference
    let mut s = String::from("hello");

    let len = reference_cal_len(&s);
    println!("s = {}, len = {}", s, len);

    // borrowing
    change(&mut s);
    println!("changed s = {}", s);
}

fn reference_cal_len(s: &String) -> usize {
    println!("in reference_cal_len, s = {}", s);
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}