extern crate rand;

use rand::prelude::*;

#[derive(Debug)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

#[derive(Debug)]
struct Card {
    suit : Suit,
    rank : usize,
}

#[derive(Debug)]
struct Deck {
    cards : Vec<Card>,
}

impl Deck {
    /// Returns a new deck of cards that is unshuffled
    fn new() -> Deck {
        let mut cards = Vec::new();
        for rank in 1..14 {
            for suit in vec![Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond] {
                cards.push(Card{rank, suit});
            }
        }
        Deck{cards}
    }

    /// Shuffle a deck in place
    /// 
    /// # Arguments
    /// * `rng` - mutable reference to a rand::Rng implementation
    fn shuffle_inplace(&mut self, mut rng: &mut impl rand::Rng) {
        self.cards.shuffle(&mut rng);
    }

    /// Draw `num` cards from the deck
    /// 
    /// # Arguments
    /// * `num` - number of cards to draw
    /// 
    /// # Returns
    /// New vec of cards drawn from the deck
    fn draw(&mut self, num: usize) -> Vec<Card> {
        let mut drawn_cards = Vec::new();
        for _ in 0..num {
            drawn_cards.push(self.cards.pop().expect("Can't draw a card!"));
        }
        drawn_cards
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut deck = Deck::new();
    deck.shuffle_inplace(&mut rng);

    let hand = deck.draw(2);

    //println!("{:?}", deck);

    println!(
        "Drew 2 cards: {:?}.\nThere are {} cards left in the deck.",
        hand, deck.cards.len()
    );

    //deck.draw(51);
}
