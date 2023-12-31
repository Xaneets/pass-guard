use crate::gui::create_vault_modal::CreateVaultModal;
use crate::{crypto, utils};

use eframe::egui;
use eframe::egui::{InnerResponse, Ui};
use egui::{FontFamily, FontId, TextStyle};
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Default, Serialize, Deserialize)]
pub struct PassGuardApp {
    master_key: String,
    path_to_vault: Option<Box<Path>>,
    sha_pass: [u8; 32],
    pub create_vault_modal: CreateVaultModal,
}
impl PassGuardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Box<Self> {
        PassGuardApp::style_init(cc);
        Box::<PassGuardApp>::default()
    }

    fn style_init(cc: &eframe::CreationContext<'_>) {
        let heading_font_size = 40.0;
        let body_font_size = 20.0;
        let monospace_font_size = 12.0;
        let button_font_size = 14.0;
        let small_font_size = 8.0;

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(heading_font_size, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(body_font_size, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(monospace_font_size, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(button_font_size, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(small_font_size, FontFamily::Proportional)),
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
            self.render_main_form(ui, ctx);
            self.dd_preview(ctx);
        });
    }

    fn render_main_form(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.horizontal_centered(|ui| {
            egui::Grid::new("form").max_col_width(250.0).show(ui, |ui| {
                ui.add_space(200.0);
                let name_label = ui.label("Master Key");
                ui.add(egui::TextEdit::singleline(&mut self.master_key).password(true))
                    .labelled_by(name_label.id);
                let into_bnt = ui.add_sized([90.0, 20.0], egui::Button::new("Into vault"));
                if into_bnt.clicked() {
                    self.check_vault_access();
                }
                ui.end_row();
                ui.add_space(200.0);
                ui.label("Vault");
                let file_button = ui.add_sized([240.0, 20.0], egui::Button::new("Choose Vault"));
                if file_button.clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("vault", &["vault"]).pick_file() {
                        self.path_to_vault = Some(path.into_boxed_path())
                    }
                }

                let create_vault_btn = ui.add_sized([90.0, 20.0], egui::Button::new("Create Vault"));
                if create_vault_btn.clicked() {
                    self.create_vault_modal.modal = true
                }
                CreateVaultModal::create_vault_modal(self, ctx);
            })
        });
    }

    fn check_vault_access(&mut self) {
        if self.master_key.is_empty() {
            return;
        }
        let path;
        match self.path_to_vault.clone() {
            None => {
                return;
            }
            Some(p) => {
                path = p;
            }
        }
        let file = File::open(path).expect("eee");
        let mut buffer = Vec::new();

        let mut reader = io::BufReader::new(file);

        reader.read_to_end(&mut buffer).expect("err");
        let mut meta: [u8; 16 + 12] = [0; 16 + 12];
        meta.copy_from_slice(&buffer[..16 + 12]);
        let mut content = vec![];
        content.extend_from_slice(&buffer[16 + 12..]);
        let hash = crypto::hash::sha256(self.master_key.as_bytes());
        let mut tag: [u8; 16] = [0; 16];
        tag.copy_from_slice(&meta[..16]);
        let mut nonce: [u8; 12] = [0; 12];
        nonce.copy_from_slice(&meta[16..]);

        let res = crypto::aes_256_gcm::Aes256Gcm::decrypt(
            content.clone(),
            hash,
            utils::unsafe_cast::bytes_as_nonce(nonce),
            *utils::unsafe_cast::bytes_as_tag(tag),
        );
        match res {
            Ok((_, _, _)) => {
                println!("Todo!")
            }
            Err(e) => {
                println!("Pass Error! {e}")
            }
        }
        ()
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
                        ui.hyperlink_to(" GitHub", utils::defs::GITHUB_LINK);
                    });
                    ui.vertical_centered(|ui| {
                        ui.hyperlink_to("🌍 Site", utils::defs::SITE_LINK);
                    });
                })
        })
    }

    fn dd_preview(&mut self, ctx: &egui::Context) {
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let text = ctx.input(|i| {
                let mut text = "Dropping files:\n".to_owned();
                for file in &i.raw.hovered_files {
                    if let Some(path) = &file.path {
                        self.path_to_vault = Some(path.clone().into_boxed_path());
                        write!(text, "\n{}", path.display()).ok();
                    } else {
                        break;
                    }
                }
                text
            });

            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("dd_file")));

            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, egui::Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                TextStyle::Monospace.resolve(&ctx.style()),
                egui::Color32::WHITE,
            );
        }
    }
}

impl eframe::App for PassGuardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_ui(ctx)
    }
}
