// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;

        (x2 - x1) * (y2 - y1)
    }
}

pub fn main() {
    let rect = Rectangle {
        top_left: Point {
            x: 0 as f32,
            y: 0 as f32,
        },
        bottom_right: Point {
            x: 10 as f32,
            y: 10 as f32,
        },
    };

    let area = rect.area();

    println!("{}", area);
}
