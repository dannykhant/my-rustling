fn main() {
    expressions();

    let v = five();
    println!("v is: {v}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("y is: {y}");
}

fn five() -> i32 {
    5 + 1
}