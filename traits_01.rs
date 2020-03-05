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

/// Defines a 2D shape
trait Shape2D {
    /// The area of the shape in 2D
    fn area(&self) -> f64;

    /// The perimeter of the shape
    fn perimeter(&self) -> f64;
}

impl Shape2D for Rectangle {
    fn area(&self) -> f64 {
        self.width() * self.height()
    }

    fn perimeter(&self) -> f64 {
        2.0 * self.width() + 2.0 * self.height()
    }
}

impl Shape2D for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
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
    println!("Rectangle's perimeter is is {}m", rect.perimeter());
    
    let circle = Circle{
        center : Point(0.0, 0.0),
        radius : 8.88,
    };
    println!("Circle is {:?}", circle);
    println!("Circle is {}m²", circle.area());
    println!("Circle's perimeter is is {}m", circle.perimeter());
}
