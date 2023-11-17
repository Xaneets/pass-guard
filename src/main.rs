use eframe::egui;
use eframe::egui::{InnerResponse, Ui};
use egui::{FontFamily, FontId, TextStyle};
use std::path::Path;

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

#[derive(Default)]
struct PassGuardApp {
    master_key: String,
    _path_to_vault: Option<Box<Path>>,
}

impl PassGuardApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Box<Self> {
        PassGuardApp::style_init(cc);
        Box::new(PassGuardApp {
            master_key: Default::default(),
            _path_to_vault: None,
        })
    }

    fn style_init(cc: &eframe::CreationContext<'_>) {
        let heading_font_size = 40.0;
        let body_font_size = 20.0;
        let monospace_font_size = 12.0;
        let button_font_size = 14.0;
        let small_font_size = 8.0;

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(heading_font_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(body_font_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(monospace_font_size, FontFamily::Monospace),
            ),
            (
                TextStyle::Button,
                FontId::new(button_font_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(small_font_size, FontFamily::Proportional),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);
    }
    fn show_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            Self::render_header(ui);
        });
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            // todo add icons
            Self::render_footer(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_form(ui);
        });
    }

    fn render_main_form(&mut self, ui: &mut Ui) {
        ui.horizontal_centered(|ui| {
            egui::Grid::new("form").max_col_width(250.0).show(ui, |ui| {
                ui.add_space(200.0);
                let name_label = ui.label("Master Key");
                ui.add(egui::TextEdit::singleline(&mut self.master_key).password(true))
                    .labelled_by(name_label.id);
                ui.end_row();
                ui.add_space(200.0);
                ui.label("Vault");
                ui.add_sized([240.0, 20.0], egui::Button::new("Choose Vault"));
            })
        });
    }

    fn render_header(ui: &mut Ui) -> InnerResponse<()> {
        ui.vertical_centered(|ui| {
            ui.heading("pass-guard.xyz");
        })
    }

    fn render_footer(ui: &mut Ui) -> InnerResponse<InnerResponse<()>> {
        ui.horizontal_centered(|ui| {
            egui::Grid::new("links")
                .num_columns(2)
                .min_col_width(utils::defs::WIN_WIDTH / 2.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.hyperlink_to("GitHub", "https://github.com/Xaneets/pass-guard");
                    });
                    ui.vertical_centered(|ui| {
                        ui.hyperlink_to("Site", "https://pass-guard.xyz");
                    });
                })
        })
    }
}

impl eframe::App for PassGuardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_ui(ctx)
    }
}
