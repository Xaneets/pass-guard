#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    title: String,
    user_name: String,
    password: String,
    url: String,
    description: String,
}
