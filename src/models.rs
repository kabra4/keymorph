use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct TextSchema {
    pub text: String,
    pub from: String,
    pub to: String,
}
