#![allow(unused)]

fn main() {
    use std::net::IpAddr;

    let home:IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP valid...");
}
