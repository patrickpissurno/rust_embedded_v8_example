use conrod_core::{
    position, position::Dimension, widget, Colorable, Position, Positionable, Sizeable, Widget,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Rectangle {
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
    pub fill: Option<bool>,
    pub outline: Option<bool>,
}

impl Rectangle {
    pub fn do_updates(&self, ui: &mut conrod_core::UiCell, ids: &HashMap<String, conrod_core::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        let mut w = {
            if let Some(true) = self.outline {
                widget::Rectangle::outline([0.0, 0.0])
            } else {
                widget::Rectangle::fill([0.0, 0.0])
            }
        };

        crate::do_updates_common!(self, w, ids);
        crate::do_updates_color!(self, w, ids);
        crate::do_updates_position!(self, w, ids);
        crate::do_updates_dimension!(self, w, ids);

        w.set(*id, ui);
    }
}
