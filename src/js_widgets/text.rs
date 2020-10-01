use conrod::{
    position, position::Dimension, widget, Colorable, Position, Positionable, Sizeable, Widget,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text {
    // shared by all widgets
    pub id: String,
    pub parent: Option<String>,
    pub no_parent: Option<bool>,
    pub graphics_for: Option<String>,
    pub floating: Option<bool>,
    pub crop_kids: Option<bool>,
    pub scroll_kids: Option<bool>,
    pub scroll_kids_vertically: Option<bool>,
    pub scroll_kids_horizontally: Option<bool>,
    pub place_on_kid_area: Option<bool>,

    // colorable
    pub color: Option<(f32, f32, f32, f32)>,

    // positionable
    pub depth: Option<f32>,
    pub x_position: Option<super::Position>,
    pub y_position: Option<super::Position>,

    // sizeable
    pub x_dimension: Option<super::Dimension>,
    pub y_dimension: Option<super::Dimension>,

    // specific to this widget
    pub text: Option<String>,
    pub font_size: Option<u32>,
    pub left_justify: Option<bool>,
    pub center_justify: Option<bool>,
    pub right_justify: Option<bool>,
    pub line_spacing: Option<f64>,
    pub no_line_wrap: Option<bool>,
    pub wrap_by_word: Option<bool>,
    pub wrap_by_character: Option<bool>,
}

impl Text {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        let txt = self.text.clone().unwrap_or("".to_owned());

        let mut w = widget::Text::new(&txt).middle_of(ui.window);

        crate::do_updates_common!(self, w, ids);
        crate::do_updates_color!(self, w, ids);
        crate::do_updates_position!(self, w, ids);
        crate::do_updates_dimension!(self, w, ids);

        w = match &self.font_size {
            Some(s) => w.font_size(*s),
            _ => w,
        };

        w = match &self.left_justify {
            Some(true) => w.left_justify(),
            _ => w,
        };

        w = match &self.center_justify {
            Some(true) => w.center_justify(),
            _ => w,
        };

        w = match &self.right_justify {
            Some(true) => w.right_justify(),
            _ => w,
        };

        w = match &self.line_spacing {
            Some(v) => w.line_spacing(*v),
            _ => w,
        };
        w = match &self.no_line_wrap {
            Some(true) => w.no_line_wrap(),
            _ => w,
        };
        w = match &self.wrap_by_word {
            Some(true) => w.wrap_by_word(),
            _ => w,
        };
        w = match &self.wrap_by_character {
            Some(true) => w.wrap_by_character(),
            _ => w,
        };

        w.set(*id, ui);
    }
}
