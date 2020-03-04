/// Suits in a standard deck
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

/// Representation of a playing card
struct Card {
    suit : Suit,
    rank : usize,
}


fn main() {
    let favourite_hand = [
        Card{suit: Suit::Spade, rank: 9},
        Card{suit: Suit::Spade, rank: 10},
    ];

    println!("{}", favourite_hand[0].rank);
}
