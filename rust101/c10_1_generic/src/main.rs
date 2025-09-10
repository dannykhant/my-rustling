#![allow(unused)]

// fn main() {
//     let num_list = vec![33, 67, 15, 91, 21];

//     let result = largest(&num_list);
//     println!("Largest num: {result}");

//     let char_list = vec!['y', 'a', 'm', 'l'];

//     let result = largest(&char_list);
//     println!("Largest char: {result}");
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for n in list {
//         if n > largest {
//             largest = n;
//         }
//     }

//     largest
// }

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<T1, U1>(self, other: Point<T1, U1>) -> Point<T, U1> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.1 };
    println!("p.x = {}", p1.x());

    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}