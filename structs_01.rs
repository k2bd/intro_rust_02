extern crate itertools;
extern crate rand;

use itertools::Itertools;
use rand::Rng;

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
    fn new() -> Deck {
        let cards = iproduct!(1..13, vec![Spade, Club, Heart, Diamond])
                    .map(|rank, suit| Card{rank, suit}).to_vec();
        Deck {cards}
    }
}

fn main() {
    let _ = Vec::<Card>::new();

    let deck = Deck::new();

    println!("{:?}", deck);
}
