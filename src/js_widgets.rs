use serde::{Deserialize};

pub mod text;

#[derive(Deserialize)]
pub enum Widget {
    Text(text::Text)
}