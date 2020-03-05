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

fn main() {
    let integer_pair = Pair { x: 5, y: 10 };
    let float_pair = Pair { x: 1.0, y: 4.0 };
    let str_pair = Pair { x: String::from("First Elem"), y: String::from("Second Elem")};
    let suit_pair = Pair { x: Suit::Heart, y: Suit::Club };
}
