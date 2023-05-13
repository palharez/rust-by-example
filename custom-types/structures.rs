#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    return (x2 - x1) * (y2 - y1);
}

fn square(point: Point, side: f32) -> Rectangle {
    let Point { x, y } = point;

    return Rectangle {
        top_left: Point { x, y: y + side },
        bottom_right: Point { x: x + side, y },
    };
}

fn main() {
    let rectangle = Rectangle {
        top_left: Point { x: 20.0, y: 10.0 },
        bottom_right: Point { x: 30.0, y: 20.0 },
    };

    println!(
        "The area of the rectangle is {} pixels.",
        rect_area(rectangle)
    );

    let point = Point { x: 10.0, y: 10.0 };
    let side = 10.0;

    println!("The square is {:?}", square(point, side));
}
