#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    show_kind(four);
    show_kind(six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
}

fn show_kind(ip_kind: IpAddrKind) {
    println!("show_kind {:?}!", ip_kind);
}
