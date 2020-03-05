
trait Foo {
    fn hello(&self) -> i32;
}

trait Bar {
    fn hello(&self) -> i32;
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

fn main() {

    // Multiple versions of hello found!
    //5.hello();
}