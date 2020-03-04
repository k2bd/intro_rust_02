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

    let my_card = Card {suit: Suit::Heart, rank: 3};

    let array_hand = [
        Card{suit: Suit::Spade, rank: 9},
        Card{suit: Suit::Spade, rank: 10},
    ];

    let tuple_hand = (
        Card{suit: Suit::Club, rank: 1},
        Card{suit: Suit::Spade, rank: 1},
    );

    let vec_hand = vec![
        Card{suit: Suit::Club, rank: 13},
        Card{suit: Suit::Heart, rank: 1},
    ];

    println!("{}", my_card.rank);
    println!("{}", array_hand[0].rank);
    println!("{}", tuple_hand.1.rank);
    println!("{}", vec_hand[0].rank);
}