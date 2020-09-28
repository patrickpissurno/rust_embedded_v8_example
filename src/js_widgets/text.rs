use conrod::{widget, Colorable, Positionable, Widget};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text {
    pub id: String,
    pub text: String,
}

impl Text {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        widget::Text::new(&self.text)
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(*id, ui);
    }
}
