fn main() {
    
    loop_var();

    loop_label();

    while_loop();

    with_collections();

    concise_for();

    reverse_range();
}

fn reverse_range() {
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
}

fn concise_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("value for: {element}");
    }
}

fn with_collections() {
    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;

    while idx < 5 {
        println!("value: {}", a[idx]);
        idx += 1;
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn loop_var() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");
}
