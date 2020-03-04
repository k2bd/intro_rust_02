
trait Foo {
    fn hello(&self) -> i32;
}

trait Bar {
    fn hello(&self) -> i32;
}

trait Baz {
    fn hello(&self) -> f64;
}


impl Foo for i32 {
    fn hello(&self) -> i32 {
        *self
    }
}

impl Bar for i32 {
    fn hello(&self) -> i32 {
        *self + 1
    }
}

impl Baz for i32 {
    fn hello(&self) -> f64 {
        *self as f64
    }
}

fn main() {

    // Multiple versions of hello found!
    //5.hello();

    println!("{}", Bar::hello(&5));
}