slint::include_modules!();

fn main() {
    if let Ok(ui) = MainConsole::new() {
        ui.set_content("Hello!! Welcome to this wolrd!\n".into());
        let _ = ui.run();
    }
}
