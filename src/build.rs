fn main() {
    slint_build::compile("ui/common_base.slint").unwrap();
    slint_build::compile("ui/console_win.slint").unwrap();
}