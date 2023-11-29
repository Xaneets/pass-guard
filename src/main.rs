#[macro_use]
extern crate serde;

use eframe::egui;
use gui::pass_guard_app::PassGuardApp;

mod crypto;
mod gui;
mod models;
mod utils;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_drag_and_drop(true)
            .with_max_inner_size(egui::vec2(utils::defs::WIN_WIDTH, utils::defs::WIN_HEIGHT))
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native("Password Guard", options, Box::new(|cc| PassGuardApp::new(cc))).expect("Fatal error!");
}
