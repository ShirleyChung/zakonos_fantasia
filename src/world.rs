use chrono::{DateTime, Local};

pub struct World {
    pub time: DateTime<Local>,
}

impl World {
    pub fn get_curr_time(&mut self) -> String {
        self.time = Local::now();
        format!("{}", self.time.format("%Y/%m/%d/ %H:%M:%S"))
    }
}