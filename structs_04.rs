/// Suits in a standard deck
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

/// Representation of a playing card
enum Card{
    Joker,
    Standard{rank: usize, suit: Suit},
}


fn main() {
    // Not necessary to specify the type of the Vec as Card
    let mut deck = Vec::<Card>::new();

    // Q: What happens if we reverse the double loop?
    for rank in 1..14 {
        for suit in vec![Suit::Heart, Suit::Spade, Suit::Diamond, Suit::Club] {
            // Note: we don't need to write `{rank: rank, suit: suit}` because
            // the names match
            deck.push(Card::Standard{rank, suit});
        }
    }

    for _ in 0..2 {
        deck.push(Card::Joker);
    }

    println!("There are {} cards in the deck.", deck.len());
}
