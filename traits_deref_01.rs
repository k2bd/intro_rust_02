use std::ops::Deref;

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

struct Deck(Vec<Card>);

impl Deck {
    /// Create a new unshuffled deck
    /// 
    /// # Arguments
    /// * `include_jokers` - if true, add two jokers at the end of the deck.
    fn new(include_jokers : bool) -> Deck {
        let mut deck = Vec::new();

        for rank in 1..14 {
            for suit in vec![Suit::Heart, Suit::Spade, Suit::Diamond, Suit::Club] {
                // Note: we don't need to write `{rank: rank, suit: suit}` because
                // the names match
                deck.push(Card::Standard{rank, suit});
            }
        }

        if include_jokers {
            for _ in 0..2 {
                deck.push(Card::Joker);
            }
        }


        Deck(deck)
    }
}

// See also: std::ops::DerefMut
impl Deref for Deck {
    type Target = Vec<Card>;

    // Signature could be written as:
    //fn deref(&self) -> &Self::Target {
    fn deref(&self) -> &Vec<Card> {
        &self.0
    }
}


fn main() {
    // Not necessary to specify the type of the Vec as Card
    let mut deck = Deck::new(true);

    // Unfortunately we've lost the ability to do this:
    println!("There are {} cards in the deck.", deck.len());
}
