use eframe::{egui, CreationContext};
use gui::pass_guard_app::PassGuardApp;

mod crypto;
mod gui;
mod utils;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        max_window_size: Some(egui::vec2(utils::defs::WIN_WIDTH, utils::defs::WIN_HEIGHT)),
        min_window_size: Some(egui::vec2(utils::defs::WIN_WIDTH, utils::defs::WIN_HEIGHT)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Password Guard",
        options,
        Box::new(|cc| PassGuardApp::new(cc)),
    )
    .expect("Fatal error!");
}
