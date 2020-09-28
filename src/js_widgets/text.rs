use conrod::{widget, Colorable, Positionable, Widget};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text {
    pub id: String,
    pub text: Option<String>,
    pub font_size: Option<u32>,
    pub color: Option<(f32, f32, f32, f32)>
}

impl Text {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        let mut color = conrod::color::WHITE;
        match self.color {
            Some(c) => {
                color = conrod::Color::Rgba(c.0, c.1, c.2, c.3);
            }
            _ => ()
        }

        widget::Text::new(&self.text.as_ref().unwrap_or(&"".to_owned()))
            .middle_of(ui.window)
            .color(color)
            .font_size(self.font_size.unwrap_or(32))
            .set(*id, ui);
    }
}
