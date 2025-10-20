fn main() {
    let num_list = vec![34, 50, 67, 23, 15];

    let result = largest(&num_list);
    println!("Largest: {result}");
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}