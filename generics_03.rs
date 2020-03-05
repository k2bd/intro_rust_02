struct Pair<T> {
    x: T,
    y: T,
}

enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

struct Rectangle {
    width : f64,
    height : f64,
}

struct Circle {
    radius : f64,
}

/// Defines a 2D shape with an area
trait Area {
    /// The area of the shape in 2D
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let integer_pair = Pair { x: 5, y: 10 };
    let float_pair = Pair { x: 1.0, y: 4.0 };
    let str_pair = Pair { x: String::from("First Elem"), y: String::from("Second Elem")};
    let suit_pair = Pair { x: Suit::Heart, y: Suit::Club };
    let suit_area: Area = Pair { x: Circle{radius: 10.0}, y: Rectangle{width: 1.0, height: 1.0}};
}
