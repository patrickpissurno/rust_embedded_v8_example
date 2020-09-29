use serde::Deserialize;
use std::collections::HashMap;

pub mod text;

pub type Scalar = f64;
pub type Margin = f64;
pub type Id = String;

#[derive(Deserialize)]
pub enum Position {
    Absolute(Scalar),
    Relative(Relative, Option<Id>)
}

#[derive(Deserialize)]
pub enum Relative {
    Scalar(Scalar),
    Align(Align),
    Direction(Direction, Scalar),
    Place(Place),
}

#[derive(Deserialize)]
pub enum Align {
    Start,
    Middle,
    End,
}

#[derive(Deserialize)]
pub enum Direction {
    Forwards,
    Backwards,
}

#[derive(Deserialize)]
pub enum Place {
    Start(Option<Margin>),
    Middle,
    End(Option<Margin>),
}

#[derive(Deserialize)]
pub enum JsWidget {
    Text(text::Text),
}

impl JsWidget {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        match self {
            JsWidget::Text(w) => {
                w.do_updates(ui, ids);
            }
        }
    }
}
