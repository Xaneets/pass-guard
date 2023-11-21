use crate::models::record::Record;
use crate::models::sub_vault::SubVault;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Vault {
    name: String,
    tag_bytes: [u8; 16],
    nonce_bytes: [u8; 12],
    records: Vec<Record>,
    sub_vaults: SubVault,
}
