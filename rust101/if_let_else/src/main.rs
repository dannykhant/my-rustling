fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("{max}"),
        _ => ()
    };
}
