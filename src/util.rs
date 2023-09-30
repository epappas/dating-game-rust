use crate::models::{Alice, Bob, Card, Deck};

pub fn dummy_util() -> bool {
    true
}

fn find_match(deck: &mut Deck, alice: Alice, bob: Bob) -> bool {
    let half_size = deck.cards.len() / 2;

    // Assume 0-based index for marked card for simplicity
    let alice_mark = if alice.secret { 0 } else { half_size };
    let bob_mark = if bob.secret { 0 } else { half_size };

    // Shuffle and exchange halves (simplified)
    deck.cards.swap(alice_mark, bob_mark);

    // Check for marked card
    let alice_found = deck.cards[alice_mark] == Card::Queen; // Assuming Queen is the marked card
    let bob_found = deck.cards[bob_mark] == Card::Queen;

    alice_found && bob_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_util_test() {
        assert!(dummy_util());
    }

    #[test]
    fn test_find_match() {
        let mut deck = Deck {
            cards: vec![Card::Queen, Card::King],
        };

        let alice = Alice { secret: true }; // Alice wants to date
        let bob = Bob { secret: true }; // Bob wants to date

        // Both want to date, should return true
        assert_eq!(find_match(&mut deck, alice, bob), true);

        let alice = Alice { secret: false }; // Alice doesn't want to date
        let bob = Bob { secret: true }; // Bob wants to date

        // Mismatch, should return false
        assert_eq!(find_match(&mut deck, alice, bob), false);

        let alice = Alice { secret: true }; // Alice wants to date
        let bob = Bob { secret: false }; // Bob doesn't want to date

        // Mismatch, should return false
        assert_eq!(find_match(&mut deck, alice, bob), false);

        let alice = Alice { secret: false }; // Alice doesn't want to date
        let bob = Bob { secret: false }; // Bob doesn't want to date

        // Both don't want to date, should return false
        assert_eq!(find_match(&mut deck, alice, bob), false);
    }
}
