pub fn debug() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let june = Person { name: "June", age: 21 };

    println!("{:?} months", 12);
    println!("{1:?} {0:?} is {job:?}", "Bond", "James", job="spy");

    println!("Struct: {:?}", Structure(3));
    println!("Another Struct: {:?}", Deep(Structure(7)));

    println!("{:#?}", june);
}