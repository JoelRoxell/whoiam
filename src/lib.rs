use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayInformation {
    pub name: String,
    pub account: String,
    pub user_id: String,
    pub arn: String,
    pub aliases: Vec<String>,
}
