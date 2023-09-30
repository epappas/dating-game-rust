#[derive(Debug, PartialEq)]
pub enum Card {
    Heart(u8),
    Diamond(u8),
    Spade(u8),
    Club(u8),
}

#[derive(Debug, Default, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Alice {
    pub secret: bool,
}

#[derive(Debug, Default, PartialEq)]
pub struct Bob {
    pub secret: bool,
}
