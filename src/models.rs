#[derive(Debug, PartialEq, Clone)]
pub enum Card {
    Queen,
    King,
}

#[derive(Clone, Debug, Default, PartialEq)]
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
    fn cyclic_shift(&mut self, shift: usize);
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
        cards.push(Card::King);
        cards.extend(other.cards.clone());

        Deck { cards }
    }
}

/// Implement Deck::cyclic_shift.
impl CyclicShiftTrait for Deck {
    fn cyclic_shift(&mut self, shift: usize) {
        assert_eq!(self.cards.len(), 5);

        self.cards.rotate_left(shift % 5);
    }
}

/// Implement Deck::decode.
impl DecodeTrait for Deck {
    fn decode(&self) -> bool {
        let mut count = 0;
        for i in 0..self.cards.len() {
            if let Card::King = self.cards[i] {
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
    fn encode(&self) -> Deck;
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
    fn encode(&self) -> Deck {
        match self.secret {
            true => Deck {
                cards: vec![Card::Queen, Card::King],
            },
            false => Deck {
                cards: vec![Card::King, Card::Queen],
            },
        }
    }
}

/// # Encode trait
/// Implement Encode trait for Bob
/// The only function needed is encode,
/// in which we accept a Boolean input and output a deck.
impl EncodeTrait for Bob {
    fn encode(&self) -> Deck {
        match self.secret {
            true => Deck {
                cards: vec![Card::King, Card::Queen],
            },
            false => Deck {
                cards: vec![Card::Queen, Card::King],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_cyclic_shift() {
        let mut deck = Deck {
            cards: vec![
                Card::Queen,
                Card::King,
                Card::Queen,
                Card::King,
                Card::Queen,
            ],
        };

        deck.cyclic_shift(2);

        assert_eq!(
            deck,
            Deck {
                cards: vec![
                    Card::Queen,
                    Card::King,
                    Card::Queen,
                    Card::Queen,
                    Card::King
                ],
            }
        );
    }

    #[test]
    fn test_game_simulation() {
        let alice = Alice { secret: true };
        let bob = Bob { secret: false };

        let alice_deck = alice.encode();
        let bob_deck = bob.encode();

        let mut joined_deck = alice_deck.join(&bob_deck);

        let mut rng = rand::thread_rng();
        joined_deck.cyclic_shift(rng.gen_range(0..5));
        joined_deck.cyclic_shift(rng.gen_range(0..5));

        assert_eq!(joined_deck.decode(), false);
    }
}
