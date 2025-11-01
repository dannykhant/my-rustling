#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { 
                    top_left: Point { x: tlx, y: tly }, 
                    bottom_right: Point {x: brx, y: bry} 
                } = rect;

    let length = brx - tlx;
    let width = bry - tly;
    (length * width).abs()
}

fn square(point: Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: point,
        bottom_right: Point { x: length, y: length }
    }
}

pub fn run() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.3, ..another_point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let area = rect_area(rectangle);
    println!("area: {}", area);

    let square = square(point, area);
    println!("{:#?}", square);
}