use serde::{Deserialize, Serialize};

/** Describe a card */
#[derive(Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub mana_cost: String,
    pub type_line: String,
    pub oracle_text: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
}
