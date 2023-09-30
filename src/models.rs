#[derive(Debug, PartialEq, Clone)]
pub enum Card {
    Queen(u8),
    King(u8),
}

#[derive(Debug, Default, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

/// # Deck::join.
///
/// This function will output a new deck
/// that is comprised of three elements:
/// two input decks and a separator king card.
pub trait JoinTrait {
    fn join(&self, other: &Deck) -> Deck;
}

/// # Deck::cyclic_shift
///
/// This function will take a deck of size
/// five and a required shift and will
/// return a deck with the cards rotated
/// to the left by shift.
pub trait CyclicShiftTrait {
    fn cyclic_shift(&self, shift: usize) -> Deck;
}

/// # Deck::decode.
///
/// The method takes self (a deck of size five)
/// as a parameter and outputs a Boolean value as
/// follows: if there are three kings in a row in
/// our set of five cards we return true.
/// Note: a king in positions 1 and 5 are considered
/// adjacent, so if you have a king in positions
/// 1, 2, and 5 we should return true.
pub trait DecodeTrait {
    fn decode(&self) -> bool;
}

/// Implement Deck::join.
impl JoinTrait for Deck {
    fn join(&self, other: &Deck) -> Deck {
        let mut cards = Vec::new();

        cards.extend(self.cards.clone());
        cards.push(Card::King(0));
        cards.extend(other.cards.clone());

        Deck { cards }
    }
}

/// Implement Deck::cyclic_shift.
impl CyclicShiftTrait for Deck {
    fn cyclic_shift(&self, shift: usize) -> Deck {
        let mut cards = Vec::new();

        for i in shift..self.cards.len() {
            cards.push(self.cards[i].clone());
        }

        for i in 0..shift {
            cards.push(self.cards[i].clone());
        }

        Deck { cards }
    }
}

/// Implement Deck::decode.
impl DecodeTrait for Deck {
    fn decode(&self) -> bool {
        let mut count = 0;
        for i in 0..self.cards.len() {
            if let Card::King(_) = self.cards[i] {
                count += 1;
            } else {
                count = 0;
            }
            if count == 3 {
                return true;
            }
        }
        false
    }
}

/// # Encode trait
///
/// Implement structs for Alice and Bob.
/// The only function needed is encode,
/// in which we accept a Boolean input and output a deck.
/// The deck should contain two cards, placed in the following orders:
///
/// | - | Yes | No  |
/// |-------|-----|-----|
/// | Alice | Queen,King | King,Queen |
/// | Bob | King,Queen | Queen,King |
///
pub trait EncodeTrait {
    fn encode(&self, input: bool) -> Deck;
}

#[derive(Debug, Default, PartialEq)]
pub struct Alice {
    pub secret: bool,
}

#[derive(Debug, Default, PartialEq)]
pub struct Bob {
    pub secret: bool,
}

/// # Implement Encode trait
/// Implement Encode trait for Alice
/// The only function needed is encode,
/// in which we accept a Boolean input and output a deck.
impl EncodeTrait for Alice {
    fn encode(&self, input: bool) -> Deck {
        let mut cards = Vec::new();
        if input {
            cards.push(Card::Queen(0));
            cards.push(Card::King(0));
        } else {
            cards.push(Card::King(0));
            cards.push(Card::Queen(0));
        }
        Deck { cards }
    }
}

/// # Encode trait
/// Implement Encode trait for Bob
/// The only function needed is encode,
/// in which we accept a Boolean input and output a deck.
impl EncodeTrait for Bob {
    fn encode(&self, input: bool) -> Deck {
        let mut cards = Vec::new();
        if input {
            cards.push(Card::King(0));
            cards.push(Card::Queen(0));
        } else {
            cards.push(Card::Queen(0));
            cards.push(Card::King(0));
        }
        Deck { cards }
    }
}
