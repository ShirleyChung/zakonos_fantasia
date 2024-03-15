use chrono::{DateTime, Local};
use std::{fmt, fs::File, io::{BufRead, BufReader, Write}};

mod avatar;
use crate::world::avatar::*;

mod card;
use crate::world::card::*;

/* 世界定義 */
#[derive(Clone)]
pub struct World {
    /* 世界時間 */
    pub time: DateTime<Local>,
    pub curr_state: String,
    /* 所有卡片 */
    pub card_mgr: CardManager,
    pub avatar_mgr: AvatarManager,
    pub on_time: fn(&mut World),
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "World: {}", self.curr_state)
    }
}

const WORLD_DATA: &str  = "world.dat";
const CARD_DATA: &str   = "cards.dat";
const AVATAR_DATA: &str = "avatar.dat";

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
		if let Ok(f) = File::open(WORLD_DATA) {
            println!("loading world..");
			let mut reader = BufReader::new(f);
            let _ = reader.read_line(&mut self.curr_state);/* self必須被標註為mut, 才能更動其內容; 故所使用的self也是 */
        }
		if let Ok(f) = File::open(CARD_DATA) {
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
		if let Ok(f) = File::open(AVATAR_DATA) {
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
    #[allow(dead_code)]
    pub fn save(&mut self) {
        if let Ok(mut f) = File::create(WORLD_DATA) {
            println!("saving world..");
            let _ = f.write_fmt(format_args!("{}\n", self.curr_state));
        }
        if let Ok(mut f) = File::create(CARD_DATA) {
            println!("saving cards..");
            for (_, card) in &self.card_mgr.card_set {
                let _ = f.write_fmt(format_args!("{}\n", card.desc));
            }
        }
        if let Ok(mut f) = File::create(AVATAR_DATA) {
            println!("saving avatar..");
            for (_, avatar) in &self.avatar_mgr.avatar_set {
                let _ = f.write_fmt(format_args!("{}\n", avatar.name));
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
            on_time: |_|{},
        };
        world.load();
        world
    }
}