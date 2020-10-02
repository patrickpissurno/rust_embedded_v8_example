use conrod_core::{
    position, position::Dimension, widget, Colorable, Position, Positionable, Sizeable, Widget, Labelable,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Button {
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
    pub enabled: Option<bool>,
    pub hover_color: Option<(f32, f32, f32, f32)>,
    pub press_color: Option<(f32, f32, f32, f32)>,
    pub label: Option<String>,
}

impl Button {
    pub fn do_updates(&self, ui: &mut conrod_core::UiCell, ids: &HashMap<String, conrod_core::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        let mut w = widget::Button::new(); //TODO: implement Image variant

        crate::do_updates_common!(self, w, ids);
        crate::do_updates_color!(self, w, ids);
        crate::do_updates_position!(self, w, ids);
        crate::do_updates_dimension!(self, w, ids);

        w = match &self.enabled {
            Some(b) => w.enabled(*b),
            _ => w,
        };

        w = match &self.hover_color {
            Some(c) => w.hover_color(conrod_core::Color::Rgba(c.0, c.1, c.2, c.3)),
            _ => w,
        };

        w = match &self.press_color {
            Some(c) => w.press_color(conrod_core::Color::Rgba(c.0, c.1, c.2, c.3)),
            _ => w,
        };

        w = match &self.label {
            Some(s) => w.label(s),
            _ => w,
        };

        w.set(*id, ui);
    }
}
