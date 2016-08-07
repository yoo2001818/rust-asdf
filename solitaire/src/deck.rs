use rand::{Rng};
use card::{Card, CardCategory};

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    fn new() -> Deck {
        Deck(Vec::new())
    }
}

pub fn shuffle_deck<'a, R: Rng>(rng: &mut R, deck: &'a Deck) -> Deck {
    let &Deck(ref deck_data) = deck;
    // Just shuffle deck, nothing more
    let mut new_deck = Vec::with_capacity(deck_data.len());
    for (i, value) in deck_data.iter().enumerate() {
        let j = rng.gen_range(0, i + 1);
        if j != i {
            new_deck.push(value.clone());
            new_deck.swap(i, j);
        } else {
            new_deck.push(value.clone());
        }
    }
    Deck(new_deck)
}

pub fn populate_deck(deck: &mut Deck) -> () {
    let &mut Deck(ref mut deck_data) = deck;
    // Put all card types in to the deck
    for category in CardCategory::iter() {
        for number in 0..13 {
            deck_data.push(Card::new(category.clone(), number));
        }
    }
}

mod test {
    use super::*;
    use rand::{thread_rng};
    #[test]
    fn test_shuffle_deck() {
        // Simple case with only one entry... Wut?
        let mut rng = thread_rng();
        let mut deck = Deck::new();
        populate_deck(&mut deck);
        let new_deck = shuffle_deck(&mut rng, &deck);
        assert_eq!(deck.0.len(), new_deck.0.len());
    }
}
