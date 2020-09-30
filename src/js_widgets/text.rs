use conrod::{widget, Colorable, position, Position, Positionable, Widget};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text {
    pub id: String,
    pub text: Option<String>,
    pub font_size: Option<u32>,
    pub color: Option<(f32, f32, f32, f32)>,
    pub x_position: Option<super::Position>,
    pub y_position: Option<super::Position>,
    pub left_justify: Option<bool>,
    pub center_justify: Option<bool>,
    pub right_justify: Option<bool>,
    pub line_spacing: Option<f64>,
    pub no_line_wrap: Option<bool>,
    pub wrap_by_word: Option<bool>,
    pub wrap_by_character: Option<bool>,
    pub parent: Option<String>,
    pub no_parent: Option<bool>,
    pub graphics_for: Option<String>,
    pub floating: Option<bool>,
}

impl Text {
    pub fn do_updates(&self, ui: &mut conrod::UiCell, ids: &HashMap<String, conrod::widget::Id>) {
        let id = ids.get(&self.id).unwrap();

        let txt = self.text.clone().unwrap_or("".to_owned());

        let mut w = widget::Text::new(&txt).middle_of(ui.window);

        w = match &self.color {
            Some(c) => w.color(conrod::Color::Rgba(c.0, c.1, c.2, c.3)),
            _ => w,
        };

        w = match &self.font_size {
            Some(s) => w.font_size(*s),
            _ => w,
        };

        w = match &self.left_justify {
            Some(true) => {
                w.left_justify()
            },
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
        
        w = match &self.parent {
            Some(id) => {
                match ids.get(id) {
                    Some(id) => w.parent(*id),
                    None => w
                }
            },
            _ => w,
        };
        
        w = match &self.no_parent {
            Some(true) => w.no_parent(),
            _ => w,
        };
        
        w = match &self.graphics_for {
            Some(id) => {
                match ids.get(id) {
                    Some(id) => w.graphics_for(*id),
                    None => w
                }
            },
            _ => w,
        };
        
        w = match &self.floating {
            Some(b) => w.floating(*b),
            _ => w,
        };

        // Works the same way as the above implementations of other attributes,
        // except this one is WAY more complicated thanks to it being a enum with
        // a bunch of enum generics. The reason I used a macro here is to avoid
        // repeating the very same logic both for x and y coordinates. There might
        // be a way to remove the need for most of this code if I can get serde's
        // remote derivation to work (https://serde.rs/remote-derive.html)
        //
        // I really need to look into it, because a macro like this one will
        // need to be implemented for each and every widget that implements the
        // Positionable trait
        macro_rules! match_position {
            ($identifier:ident) => {{
                match &self.$identifier {
                    Some(p) => {
                        match p {
                            super::Position::Absolute(a) => w = w.$identifier(Position::Absolute(*a)),
                            super::Position::Relative(r, id) => {
                                let id = match id {
                                    Some(id) => Some(*ids.get(id).unwrap()),
                                    None => None
                                };
        
                                match r {
                                    super::Relative::Scalar(s) => w = w.$identifier(Position::Relative(position::Relative::Scalar(*s), id)),
                                    super::Relative::Align(a) => {
                                        w = w.$identifier(Position::Relative(position::Relative::Align(match a {
                                            super::Align::Start => position::Align::Start,
                                            super::Align::Middle => position::Align::Middle,
                                            super::Align::End => position::Align::End
                                        }), id))
                                    },
                                    super::Relative::Direction(d, s) => {
                                        w = w.$identifier(Position::Relative(position::Relative::Direction(match d {
                                            super::Direction::Forwards => position::Direction::Forwards,
                                            super::Direction::Backwards => position::Direction::Backwards
                                        }, *s), id))
                                    },
                                    super::Relative::Place(p) => {
                                        w = w.$identifier(Position::Relative(position::Relative::Place(match p {
                                            super::Place::Start(m) => position::Place::Start(*m),
                                            super::Place::Middle => position::Place::Middle,
                                            super::Place::End(m) => position::Place::End(*m),
                                        }), id))
                                    },
                                }
                            }
                        }
                    },
                    _ => (),
                }
            }};
        }

        // just calling the macro to generate code for both
        // the X and Y coordinates
        match_position!(x_position);
        match_position!(y_position);

        w.set(*id, ui);
    }
}
