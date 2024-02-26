use std::{thread, time};
slint::include_modules!();

mod world;
use crate::world::*;

/// 以訊息迴圈輸入到Window裡
fn run_event_loop(ui: slint::Weak<MainConsole>) {
        // 開始一個執行緒:
        let _ = std::thread::spawn(move || {
            let mut world = World::new();
            loop {
                // 複製一份弱參考，因所有權會被帶到invoke裡
                let ui_copy = ui.clone();
                let cur_time = world.get_curr_time();
                (world.on_time)();
                // 使用slint的invoke_from_event_loop, 來更新UI
                // 因為有move，所以所有被捕捉的變數所有權都轉移到closure裡
                let _ = slint::invoke_from_event_loop(move || {
                    let u = ui_copy.unwrap();
                    let mut status = u.get_status();
                    status.date_time = cur_time.into();                    
                    u.set_status(status);
                });
                // sleep一下再繼續下一個迴圈
                thread::sleep(time::Duration::from_millis(1000));
            }            
        });
}

fn main() {
    if let Ok(ui) = MainConsole::new() {
        run_event_loop(ui.as_weak());
        let _ = ui.run();
    }
}
