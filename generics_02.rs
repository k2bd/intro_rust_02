/// A point in 2D
#[derive(Debug)]
struct Point(f64, f64);

/// A rectangle
#[derive(Debug)]
struct Rectangle(Point, Point);

/// A circle
#[derive(Debug)]
struct Circle {
    center : Point,
    radius : f64,
}

impl Rectangle {
    /// Rectangle width
    fn width(&self) -> f64 {
        ((self.0).0 - (self.1).0).abs()
    }

    /// Rectangle height
    fn height(&self) -> f64 {
        ((self.0).1 - (self.1).1).abs()
    }
}

/// Defines a 2D shape with an area
trait Area {
    /// The area of the shape in 2D
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// Get a unit circle or a unit square
/// 
/// Arguments
/// `switch` - if `true`, return a circle, otherwise return a rectangle
fn get_unit_shape(switch: bool) -> impl Area {
    if switch {
        Circle {
            center : Point(0.0, 0.0),
            radius : 1.0,
        }
    } else {
        Rectangle(
            Point(0.0, 0.0),
            Point(1.0, 1.0),
        )
    }
}

fn main() {
    let shape = get_unit_shape(true);
    println!("My shape is {:?}", shape);
    println!("Area of my shape: {}", shape.area());
}
