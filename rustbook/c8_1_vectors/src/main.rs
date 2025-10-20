fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Third: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third: {third}"),
        None => println!("No Third!")
    };

    let mut v2 = vec![1, 3, 5];
    for i in &mut v2 {
        *i += 10;
    }

    for i in &v2 {
        println!("{i}");
    }

    // Multiple Types Vector with Enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    };
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.14)
    ];
}
