fn main() {
    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("inner scope is: {x}");
    }

    println!("x is: {x}");

    // Data-types: Scalar
    let num: u32 = 5;
    let dec: f64 = 3.14;
    let flag: bool = true;
    let c: char = 'z';

    println!("{num} {dec} {flag} {c}");

    // Data-types: Compound
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // Destructuring

    let one = tup.2; // Accessing one element

    println!("y is: {y}");

    // Array
    let arr: [u32; 3] = [1, 2, 3];

    let a = [3; 5];

    let first = a[0];
}
