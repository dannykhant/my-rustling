fn main() {

    if_expression();

}

fn if_expression() {
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("num: {number}");
}
