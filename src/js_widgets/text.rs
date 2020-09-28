use conrod::{widget, Colorable, Positionable, Widget};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text {
    pub id: String,
    pub text: Option<String>,
    pub font_size: Option<u32>
}

impl Text {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        widget::Text::new(&self.text.as_ref().unwrap_or(&"".to_owned()))
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(self.font_size.unwrap_or(32))
            .set(*id, ui);
    }
}
