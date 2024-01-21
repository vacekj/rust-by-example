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
    /// Calculates the area of the rectangle
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;

        (x2 - x1) * (y2 - y1)
    }
}

/// Creates a square Rectangle at a specified point, with a specified width and height
fn square(point: Point, side_length: f32) -> Rectangle {
    Rectangle {
        bottom_right: Point {
            x: &point.x + &side_length,
            y: &point.y + &side_length,
        },
        top_left: point,
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

#[cfg(test)]
mod tests {
    use crate::ch3custom_types::{square, Point, Rectangle};

    #[test]
    fn area() {
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

        assert_eq!(area, 100f32);
    }

    #[test]
    fn square_test() {
        let point = Point { x: 10f32, y: 10f32 };
        let sq = square(point, 10f32);

        assert_eq!(sq.bottom_right.x, 20f32);
        assert_eq!(sq.bottom_right.y, 20f32);
        assert_eq!(sq.top_left.x, 10f32);
        assert_eq!(sq.top_left.x, 10f32);
    }
}
