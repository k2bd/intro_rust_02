extern crate rand;

use rand::prelude::*;

/// Suits in a standard deck
#[derive(Debug)]
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

/// Representation of a playing card
#[derive(Debug)]
enum Card{
    Joker,
    Standard{rank: usize, suit: Suit},
}

/// A deck of playing cards
struct Deck(Vec<Card>);

impl Deck {

    /// Create a new unshuffled deck
    /// 
    /// # Arguments
    /// * `include_jokers` - if `true`, add two jokers at the end of the deck.
    fn new(include_jokers : bool) -> Deck {
        let mut deck = Vec::new();

        for rank in 1..14 {
            for suit in vec![Suit::Heart, Suit::Spade, Suit::Diamond, Suit::Club] {
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

    /// Shuffle a deck inplace
    /// 
    /// # Arguments
    /// * `rng` - Mutable reference to Rng for the shuffle
    fn shuffle_inplace(&mut self, mut rng: &mut impl rand::Rng) {
        // Vec.shuffle is provided by the rand crate
        self.0.shuffle(&mut rng);
    }

    /// Draw cards, removing them from the deck
    /// 
    /// # Arguments
    /// * `num` - Number of cards to draw
    fn draw(&mut self, num: usize) -> Vec<Card> {
        let mut hand = Vec::new();

        for _ in 0..num {
            // Note that because Deck is a tuple struct we need to reference
            // the 0'th element. We can fix this later.
            hand.push(
                self.0.pop()
                      .expect("No more cards in the deck!")
            );
        }

        hand
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut deck = Deck::new(true);

    deck.shuffle_inplace(&mut rng);

    let hand = deck.draw(2);

    println!("Hand: {:?}", hand);

    // Unfortunately we've lost the ability to do this:
    //println!("There are {} cards in the deck.", deck.len());

    // We can do this instead, for now.
    println!("There are {} cards in the deck.", deck.0.len());
    
    // We could also implement a method called len, but there must be a better way 🤔
}
