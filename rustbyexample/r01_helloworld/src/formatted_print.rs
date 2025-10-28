pub fn formatted_print() {
    // pos args
    println!("{0} comes before {1} and this is {0}", "A", "B");

    // named args
    println!("{sbj} {verb} {obj}", verb="Add", obj="You", sbj="I");

    // diff formatting
    println!("binary: {:b}", 69420);
    println!("octal: {:o}", 69420);
    println!("hexadecimal: {:x}", 69420);

    // right justify
    println!("right justify {n:>5}", n=1);

    // pad number
    println!("pad num right {n:0>5}", n=1);
    println!("pad num left {n:0<5}", n=1);

    // using named args in format specifier
    println!("named args in format {n:0>w$}", n=1, w=5);

    // capture arg from variable
    let num: f64 = 1.0;
    let width: usize = 5;
    println!("arg from var {num:>width$}");

    // pi
    let pi: f64 = 3.142;
    println!("PI is {pi:>5}");
}