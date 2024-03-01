use chrono::{DateTime, Local};
use std::{collections::HashMap, fmt, fs::File, io::{BufRead, BufReader}};

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
    avatar_set: HashMap<String, Avatar>,
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
    card_set: HashMap<String, Card>,
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

type Callback = fn();

/* 世界定義 */
#[derive(Clone)]
pub struct World {
    /* 世界時間 */
    pub time: DateTime<Local>,
    pub curr_state: String,
    /* 所有卡片 */
    pub card_mgr: CardManager,
    pub avatar_mgr: AvatarManager,
    pub on_time: Callback,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "World: {}", self.curr_state)
    }
}

/* 世界運作的函式 */
impl World {
    /* 取得並更新世界時間 */
    #[allow(dead_code)]
    pub fn get_curr_time(&mut self) -> String {
        self.time = Local::now();
        format!("{}", self.time.format("%Y/%m/%d/ %H:%M:%S"))
    }
    /* 載入先前狀態 */
    pub fn load(&mut self) {
		if let Ok(f) = File::open("world.dat") {
            println!("loading world..");
			let mut reader = BufReader::new(f);
            let _ = reader.read_line(&mut self.curr_state);/* self必須被標註為mut, 才能更動其內容; 故所使用的self也是 */
        }
		if let Ok(f) = File::open("cards.dat") {
            println!("loading cards..");
			let mut reader = BufReader::new(f);
            let mut line = String::new();
            loop { 
                if let Ok(_) = reader.read_line(&mut line) {
                    self.card_mgr.add_card(&line);
                } else {
                    break;
                }
            }
        }
		if let Ok(f) = File::open("avatar.dat") {
            println!("loading avatar..");
			let mut reader = BufReader::new(f);
            let mut line = String::new();
            if let Ok(_) = reader.read_line(&mut line) {
                self.avatar_mgr.add_avatar(&line);
            } else {
                println!("loading avatar fail");                
            }
        }
    }
    /* 世界，初始化 */
    pub fn new() -> World {
        println!("world initializing..");
        let mut world = World {
            time: Local::now(), 
            curr_state: "".to_string(),
            card_mgr: CardManager::new(),
            avatar_mgr: AvatarManager::new(),
            on_time: ||{},
        };
        world.load();
        world
    }
}