struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {

    // Could also be:
    // fn new(x: T, y: T) -> Pair<T> {
    fn new(x: T, y: T) -> Self {

        // Could also be:
        // Pair::<T> {x, y}
        Self {x, y}
    }
}

impl Pair<f64> {

    /// Print the distance from the origin of a pair of floats, interpreted as
    /// coordinates
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

fn main() {
    let integer_pair = Pair { x: 5, y: 10 };
    let float_pair = Pair::new(1.0, 4.0);
    let str_pair = Pair {x: String::from("First Elem"), y: String::from("Second Elem")};
    let suit_pair = Pair::new(Suit::Heart, Suit::Club);
    
    println!("Float pair is {} from the origin", float_pair.distance_from_origin());

    // No such method!
    //println!("Int pair is {} from the origin", integer_pair.distance_from_origin());
}
