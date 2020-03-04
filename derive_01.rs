use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

fn main() {
    let mut map = HashMap::new();

    map.insert(Point(10, 10), 60);
    map.insert(Point(1, -100), 0);

    println!("{:?}", map);
}
