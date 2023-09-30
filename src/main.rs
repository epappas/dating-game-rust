mod models;

use models::{Alice, Bob, CyclicShiftTrait, EncodeTrait, JoinTrait};
use std::io;

use crate::models::DecodeTrait;

// Function to get user input
fn input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}

fn main() {
    // Print welcome message
    println!("Welcome to the Matching Game!");

    // Prompt player 1 (Alice)
    println!("Player 1: Do you want to match with Player 2? (yes/no)");
    let alice_input = input();

    // Prompt player 2 (Bob)
    println!("Player 2: Do you want to match with Player 1? (yes/no)");
    let bob_input = input();

    // Instantiate Alice and Bob
    let alice = Alice {
        secret: alice_input == "yes",
    };
    let bob = Bob {
        secret: bob_input == "yes",
    };

    // Encode and Join Decks
    let alice_deck = alice.encode();
    let bob_deck = bob.encode();
    let joined_deck = alice_deck.join(&bob_deck);

    // Shuffle decks
    println!("Player 1: Input the number of cyclic shifts:");
    let alice_shifts: usize = input().trim().parse().expect("Please input a number.");
    let mut shuffled_deck = joined_deck.clone();
    shuffled_deck.cyclic_shift(alice_shifts);

    println!("Player 2: Input the number of cyclic shifts:");
    let bob_shifts: usize = input().trim().parse().expect("Please input a number.");
    shuffled_deck.cyclic_shift(bob_shifts);

    // Decode and Output
    let match_found = shuffled_deck.decode();
    let result = if match_found { "Match" } else { "No Match" };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn dummy_test() {
        assert!(true);
    }
}
