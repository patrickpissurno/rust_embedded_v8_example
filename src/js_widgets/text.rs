use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Text {
    pub id: String,
    pub text: String,
}