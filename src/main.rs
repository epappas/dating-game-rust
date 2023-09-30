mod models;
mod util;

use util::dummy_util;

fn main() {
    dummy_util();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn dummy_test() {
        assert!(true);
    }
}
