use std::{collections::HashMap, fmt};

/* 卡片定義 */
#[derive(Clone)]
pub struct Card {
    pub desc: String,
    pub power: i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card desc = {}", self.desc)
    }
}

impl Card {
    #[allow(dead_code)]
    fn new(desc: String) -> Card {
        Card {
            desc: desc,
            power: 0,
        }
    }
}

/* 卡片管理員 */
#[derive(Clone)]
pub struct CardManager {
    pub card_set: HashMap<String, Card>,
}

impl CardManager {
    #[allow(dead_code)]
    pub fn get_card(&mut self, id: &'static str) -> &Card {
        self.card_set.entry(id.to_string()).or_insert_with(|| Card::new(id.to_string()) )
    }
    pub fn new() -> CardManager {
        CardManager {
            card_set: HashMap::<String, Card>::new(),
        }
    }
    pub fn add_card(&mut self, desc: &String) {
        let key = format!("{}", fastrand::i32(..));
        self.card_set.insert(key.clone(), Card::new(desc.clone()));
    }
}
