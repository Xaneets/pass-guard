use crate::crypto::aes_256_gcm::NonceSeq;
use crate::gui::pass_guard_app::PassGuardApp;
use crate::{crypto, utils};

use eframe::egui;
use eframe::egui::Context;
use rand::Rng;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CreateVaultModal {
    pub modal: bool,
    pub modal_tabs: CreateVaultTabs,
    pub master_key_1: String,
    pub master_key_2: String,
    pub path_to_new_vault: Option<Box<Path>>,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum CreateVaultTabs {
    General,
    Test,
}

impl Default for CreateVaultTabs {
    fn default() -> Self {
        Self::General
    }
}

impl CreateVaultModal {
    pub fn create_vault_modal(app: &mut PassGuardApp, ctx: &Context) {
        egui::Window::new("Create a new Vault")
            .resizable(false)
            .collapsible(false)
            .min_width(utils::defs::WIN_WIDTH / 2.0)
            .min_height(utils::defs::WIN_HEIGHT / 2.0)
            .open(&mut app.create_vault_modal.modal)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    // ui.group(|ui| {
                    ui.selectable_value(&mut app.create_vault_modal.modal_tabs, CreateVaultTabs::General, "General");
                    ui.selectable_value(&mut app.create_vault_modal.modal_tabs, CreateVaultTabs::Test, "Test");
                    // })
                });
                ui.separator(); // todo replace space
                match app.create_vault_modal.modal_tabs {
                    CreateVaultTabs::General => {
                        egui::Grid::new("create-vault-grid")
                            .num_columns(3)
                            .min_row_height(utils::defs::WIN_HEIGHT / (2.0 * 8.0))
                            .max_col_width(utils::defs::WIN_WIDTH / (2.0 * 3.0))
                            .show(ui, |ui| {
                                ui.add_space(200.0);
                                let create_vault_file_btn = ui.button("create Vault file");
                                if create_vault_file_btn.clicked() {
                                    if let Some(path) = rfd::FileDialog::new().add_filter("vault", &["vault"]).save_file() {
                                        app.create_vault_modal.path_to_new_vault = Some(path.into_boxed_path());
                                    }
                                }
                                ui.end_row();
                                ui.add_space(80.0);
                                let name_label = ui.label("Master Key");
                                ui.add(egui::TextEdit::singleline(&mut app.create_vault_modal.master_key_1).password(true))
                                    .labelled_by(name_label.id);
                                ui.end_row();
                                ui.add_space(80.0);
                                let name_label = ui.label("Master Key");
                                ui.add(egui::TextEdit::singleline(&mut app.create_vault_modal.master_key_2).password(true))
                                    .labelled_by(name_label.id);
                                ui.end_row();
                                ui.add_space(200.0);
                                let crypt_btn = ui.button("Let's encrypt");
                                if crypt_btn.clicked() {
                                    if app.create_vault_modal.master_key_1 == app.create_vault_modal.master_key_2 {
                                        if app.create_vault_modal.path_to_new_vault.is_some() {
                                            let mut f = File::create(app.create_vault_modal.path_to_new_vault.clone().unwrap())
                                                .expect("Create File Error!");
                                            let hash = crypto::hash::sha256(app.create_vault_modal.master_key_1.as_bytes());
                                            let mut rng = rand::thread_rng();
                                            let random_u64: u64 = rng.gen();

                                            let (_, tag, seq) = crypto::aes_256_gcm::Aes256Gcm::encrypt(vec![], hash, NonceSeq(random_u64))
                                                .expect("Encrypt error!");

                                            // write metadata ( `Tag` + `NonceSequence` )
                                            let tag = utils::unsafe_cast::tag_as_bytes(&tag);
                                            let mut metadata: [u8; 16 + 12] = [0; 16 + 12];
                                            metadata[..16].copy_from_slice(tag);
                                            metadata[16..].copy_from_slice(&seq.as_bytes());
                                            f.write_all(&metadata).expect("Write file Error");
                                        };
                                    } else {
                                        // todo popup
                                        println!("Pass err")
                                    }
                                }
                            });
                    }
                    CreateVaultTabs::Test => {
                        ui.label("test");
                        ui.label("test");
                    }
                }
            });
    }
}
