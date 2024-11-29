use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub amount: i8
}