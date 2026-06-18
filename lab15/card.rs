enum Color {
    Karo, Kier, Pik, Trelf
}

enum Value {
    Number(i32), Walet, Dama, Krol, As
}

pub struct Card {
    color: Color,
    value: Value,
}

pub struct Deck {
    cards: Vec<Card>
}

impl Card {
    pub fn value(&self) -> i32 {
        match self.value {
            Value::Walet => 2,
            Value::Dama => 3,
            Value::Krol => 4,
            Value::As => 11,
            Value::Number(n) => n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.size(), 52);
    }
}
