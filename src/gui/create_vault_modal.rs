#[derive(Default)]
pub struct CreateVaultModal {
    pub modal: bool,
    pub modal_tabs: CreateVaultTabs,
    pub master_key_1: String,
    pub master_key_2: String,
}

#[derive(PartialEq)]
pub enum CreateVaultTabs {
    General,
    Test,
}

impl Default for CreateVaultTabs {
    fn default() -> Self {
        Self::General
    }
}
