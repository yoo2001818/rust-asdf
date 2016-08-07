use std::fmt;

#[derive(PartialEq, Debug)]
pub enum CardColor {
    BLACK,
    RED,
}

trait HasColor {
    fn get_color(&self) -> CardColor;
}

#[derive(PartialEq)]
pub enum CardCategory {
    SPADE,
    DIAMOND,
    CLUB,
    HEART,
}

impl HasColor for CardCategory {
    fn get_color(&self) -> CardColor {
        match *self {
            CardCategory::SPADE => CardColor::BLACK,
            CardCategory::DIAMOND => CardColor::RED,
            CardCategory::CLUB => CardColor::BLACK,
            CardCategory::HEART => CardColor::RED
        }
    }
}

impl fmt::Display for CardCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CardCategory::SPADE => "♠",
            CardCategory::DIAMOND => "◆",
            CardCategory::CLUB => "♣",
            CardCategory::HEART => "♥"
        })
    }
}

const CARD_NUMBERS: [&'static str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

#[derive(PartialEq)]
pub struct Card {
    category: CardCategory,
    number: u8
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.category, CARD_NUMBERS[self.number as usize])
    }
}

mod test {
    #[test]
    fn color_test() {
        // Quite obvious, however, I wanted to make sure if this works as I desired.
        assert_eq!(CardCategory::SPADE.get_color(), CardColor::BLACK);
        assert_eq!(CardCategory::DIAMOND.get_color(), CardColor::RED);
        assert_eq!(CardCategory::CLUB.get_color(), CardColor::BLACK);
        assert_eq!(CardCategory::HEART.get_color(), CardColor::RED);
    }
}
