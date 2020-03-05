use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {

    // Could also be:
    // fn new(x: T, y: T) -> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

fn main() {
    let a = Pair::new(1, 2);
    a.cmp_display();

    let b = Pair::new(String::from("A string"), String::from("Another string"));
    b.cmp_display();

    let c = Pair::new(Suit::Spade, Suit::Diamond);
    //c.cmp_display()
}
