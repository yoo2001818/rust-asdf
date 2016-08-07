use std::fmt;
use std::slice::Iter;
use self::CardCategory::*;

#[derive(PartialEq, Debug)]
pub enum CardColor {
    Black,
    Red,
}

trait HasColor {
    fn get_color(&self) -> CardColor;
}

#[derive(PartialEq, Clone)]
pub enum CardCategory {
    Spade,
    Diamond,
    Club,
    Heart,
}

impl CardCategory {
    pub fn iter() -> Iter<'static, CardCategory> {
        static CATEGORIES: [CardCategory;  4] = [Spade, Diamond, Club, Heart];
        CATEGORIES.into_iter()
    }
}

impl HasColor for CardCategory {
    fn get_color(&self) -> CardColor {
        match *self {
            CardCategory::Spade => CardColor::Black,
            CardCategory::Diamond => CardColor::Red,
            CardCategory::Club => CardColor::Black,
            CardCategory::Heart => CardColor::Red
        }
    }
}

impl fmt::Display for CardCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CardCategory::Spade => "♠",
            CardCategory::Diamond => "◆",
            CardCategory::Club => "♣",
            CardCategory::Heart => "♥"
        })
    }
}

impl fmt::Debug for CardCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

const CARD_NUMBERS: [&'static str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

#[derive(PartialEq, Clone)]
pub struct Card {
    category: CardCategory,
    number: u8
}

impl Card {
    pub fn new(category: CardCategory, number: u8) -> Card {
        Card {
            category: category,
            number: number
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.category, CARD_NUMBERS[self.number as usize])
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

mod test {
    use super::{CardCategory, CardColor, HasColor};
    #[test]
    fn color_test() {
        // Quite obvious, however, I wanted to make sure if this works as I desired.
        assert_eq!(CardCategory::Spade.get_color(), CardColor::Black);
        assert_eq!(CardCategory::Diamond.get_color(), CardColor::Red);
        assert_eq!(CardCategory::Club.get_color(), CardColor::Black);
        assert_eq!(CardCategory::Heart.get_color(), CardColor::Red);
    }
}
