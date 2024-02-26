use chrono::{DateTime, Local};
use std::{fs::File, io::{BufRead, BufReader}};

pub struct Avatar {
    pub name: String,
}

/* 卡片定義 */
#[derive(Clone)]
pub struct Card {
    pub desc: String,
}

impl Card {
    fn new(desc: &str) -> Card {
        Card {
            desc: desc.to_string(),
        }
    }
}

type Callback = fn();

/* 世界定義 */
pub struct World {
    /* 世界時間 */
    pub time: DateTime<Local>,
    pub curr_state: String,
    /* 所有卡片 */
    pub cards: Vec<Card>,
    pub player: Avatar,
    pub on_time: Callback,
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
                    self.cards.push(Card::new(&line));
                } else {
                    break;
                }
            }
        }
		if let Ok(f) = File::open("avatar.dat") {
            println!("loading avatar..");
			let mut reader = BufReader::new(f);
            let _ = reader.read_line(&mut self.player.name);
        }
    }
    /* 世界，初始化 */
    pub fn new() -> World {
        println!("world initializing..");
        let mut world = World {
            time: Local::now(), 
            curr_state: "".to_string(),
            cards: Vec::<Card>::new(),
            player: Avatar {name: "player".to_string()},
            on_time: ||{},
        };
        world.load();
        world
    }
}