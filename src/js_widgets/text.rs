use conrod::{widget, Colorable, Position, Positionable, Widget};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text {
    pub id: String,
    pub text: Option<String>,
    pub font_size: Option<u32>,
    pub color: Option<(f32, f32, f32, f32)>,
    pub x_position: Option<Pos>,
    pub y_position: Option<Pos>,
}

#[derive(Deserialize)]
pub enum Pos {
    Absolute(f64),
}

impl Text {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        let txt = self.text.clone().unwrap_or("".to_owned());

        let mut w = widget::Text::new(&txt).middle_of(ui.window);

        match &self.color {
            Some(c) => w = w.color(conrod::Color::Rgba(c.0, c.1, c.2, c.3)),
            _ => (),
        }

        match &self.font_size {
            Some(s) => w = w.font_size(*s),
            _ => (),
        }

        match &self.x_position {
            Some(p) => {
                match p {
                    Pos::Absolute(a) => w = w.x_position(Position::Absolute(*a))
                }
            },
            _ => (),
        }

        match &self.y_position {
            Some(p) => {
                match p {
                    Pos::Absolute(a) => w = w.y_position(Position::Absolute(*a))
                }
            },
            _ => (),
        }

        w.set(*id, ui);
    }
}
