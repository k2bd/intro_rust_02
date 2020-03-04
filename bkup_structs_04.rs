/// Suits in a standard deck
enum Suit {
    Heart{name: String},
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

    for rank in 1..14 {
        for suit in vec![Suit::Heart{name: String::from("hello")}, Suit::Spade, Suit::Diamond, Suit::Club] {
            deck.push(Card::Standard{rank, suit});
        }
    }
    for _ in 0..2 {
        deck.push(Card::Joker);
    }

    println!("There are {} cards in the deck.", deck.len());
    for card in deck {
        if let Card::Standard{suit: Suit::Heart{name: x}, rank: _} = card {
            println!("{:p}", &x);
        }
    }
}