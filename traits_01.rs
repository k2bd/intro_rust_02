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

fn main() {
    let rect = Rectangle(
        Point(-0.3, 0.6),
        Point(11.1, -55.9),
    );
    println!("Rectangle is {:?}", rect);
    println!("Rectangle is {}m by {}m", rect.width(), rect.height());
    println!("Rectangle is {}m²", rect.area());
    
    let circle = Circle{
        center : Point(0.0, 0.0),
        radius : 8.88,
    };
    println!("Circle is {:?}", circle);
    println!("Circle is {}m²", circle.area());
}
