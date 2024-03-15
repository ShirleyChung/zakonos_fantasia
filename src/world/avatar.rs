use std::{collections::HashMap, fmt};

#[derive(Clone)]
pub struct Avatar {
    pub name: String,
    pub hold_cards: Vec<String>,
}

impl Avatar {
    pub fn new(name: String) -> Avatar {
        Avatar {
            name: name,
            hold_cards: Vec::<String>::new(),
        }
    }
}

impl fmt::Display for Avatar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Avatar name = {}", self.name)
    }
}

/* 人物管理員 */
#[derive(Clone)]
pub struct AvatarManager {
    pub avatar_set: HashMap<String, Avatar>,
}

impl AvatarManager {
    #[allow(dead_code)]
    pub fn get_avatar(&mut self, id: &'static str) -> &Avatar {
        self.avatar_set.entry(id.to_string()).or_insert_with(|| Avatar::new(id.to_string()))
    }
    pub fn new() -> AvatarManager {
        AvatarManager {
            avatar_set: HashMap::<String, Avatar>::new(),
        }
    }
    pub fn add_avatar(&mut self, id: &String) {
        let key = format!("{}", fastrand::i32(..));
        self.avatar_set.insert(key.clone(), Avatar::new(id.clone()));
    }
}