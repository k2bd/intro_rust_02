enum Suit{
    Spade,
    Heart,
    Club,
    Diamond,
}

enum Card{
    Joker,
    Standard{rank: usize, suit: Suit},
}

fn main() {
    let _card1 = Card::Joker;
    let _card2 = Card::Standard{suit: Suit::Spade, rank: 5};
}
