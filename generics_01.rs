use std::fmt::Debug;

/// A point in 2D
#[derive(Debug)]
struct Point(f64, f64);

/// A rectangle
#[derive(Debug)]
struct Rectangle(Point, Point);

/// A circle
// N.B. Removed Debug
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

/// Prints the first of the two shapes in debug mode and then returns the
/// absolute difference in area between two shapes
fn area_diff_1(shape1: &(impl Area + Debug), shape2: &impl Area) -> f64 {
    println!("The first shape is {:?}", shape1);
    (shape1.area() - shape2.area()).abs()
}

/// Prints the first of the two shapes in debug mode and then returns the
/// absolute difference in area between two shapes, using trait bounds
fn area_diff_2<T: Debug + Area, U: Area>(shape1: &T, shape2: &U) -> f64 {
    println!("The first shape is {:?}", shape1);
    (shape1.area() - shape2.area()).abs()
}

/// Prints the first of the two shapes in debug mode and then returns the
/// absolute difference in area between two shapes, using clean trait bounds
/// with `where`
fn area_diff_3<T, U>(shape1: &T, shape2: &U) -> f64 
        where T: Debug + Area,
              U: Area {
    println!("The first shape is {:?}", shape1);
    (shape1.area() - shape2.area()).abs()
}


fn main() {
    let rect = Rectangle(
        Point(-0.3, 0.6),
        Point(11.1, -55.9),
    );
    
    let circle = Circle{
        center : Point(0.0, 0.0),
        radius : 8.88,
    };

    // Because in this example Circle is not Debug, we couldn't swap these two
    println!("The difference in areas is {:?}", area_diff_1(&rect, &circle));
    println!("The difference in areas is {:?}", area_diff_2(&rect, &circle));
    println!("The difference in areas is {:?}", area_diff_3(&rect, &circle));
}
