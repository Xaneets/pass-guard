use crate::models::record::Record;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SubVault {
    name: String,
    records: Vec<Record>,
}
