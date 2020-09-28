use serde::Deserialize;
use std::collections::HashMap;

pub mod text;

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
