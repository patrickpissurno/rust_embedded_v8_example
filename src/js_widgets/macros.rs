/**
 * This file includes macros that implement common behavior for
 * do_updates(), which can then be shared by different widgets
 * without source-code duplication.
 * 
 * If you have any tips on how to replace them with actual functions
 * I'm open to discussion, as I'd rather use functions and not
 * macros.
 * 
 * Macros that start with an underscore are internal (private) and
 * should not be used outside of this file. They still need to be
 * exported, tough, or the compiler won't be able to find them.
 */

// Works the same way as the implementations of other attributes,
// except this one is WAY more complicated thanks to it being a enum with
// a bunch of enum generics. The reason I used another macro here is to avoid
// repeating the very same logic both for x and y coordinates. There might
// be a way to remove the need for most of this code if I can get serde's
// remote derivation to work (https://serde.rs/remote-derive.html)
//
// I really might need to look into it, because a macro like this one will
// need to be implemented for each and every widget that implements the
// Positionable trait
#[macro_export]
macro_rules! _do_updates_position_inner {
    ($self:ident, $w:ident, $ids:ident, $field:ident) => {{
        match &$self.$field {
            Some(p) => {
                match p {
                    super::Position::Absolute(a) => $w = $w.$field(Position::Absolute(*a)),
                    super::Position::Relative(r, id) => {
                        let id = match id {
                            Some(id) => Some(*$ids.get(id).unwrap()),
                            None => None
                        };

                        match r {
                            super::Relative::Scalar(s) => $w = $w.$field(Position::Relative(position::Relative::Scalar(*s), id)),
                            super::Relative::Align(a) => {
                                $w = $w.$field(Position::Relative(position::Relative::Align(match a {
                                    super::Align::Start => position::Align::Start,
                                    super::Align::Middle => position::Align::Middle,
                                    super::Align::End => position::Align::End
                                }), id))
                            },
                            super::Relative::Direction(d, s) => {
                                $w = $w.$field(Position::Relative(position::Relative::Direction(match d {
                                    super::Direction::Forwards => position::Direction::Forwards,
                                    super::Direction::Backwards => position::Direction::Backwards
                                }, *s), id))
                            },
                            super::Relative::Place(p) => {
                                $w = $w.$field(Position::Relative(position::Relative::Place(match p {
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

// the same thing as positions above, but for dimensions
#[macro_export]
macro_rules! _do_updates_dimension_inner {
    ($self:ident, $w:ident, $ids:ident, $field:ident) => {{
        match &$self.$field {
            Some(p) => {
                match p {
                    super::Dimension::Absolute(a) => $w = $w.$field(Dimension::Absolute(*a)),
                    super::Dimension::Of(id, v) => {
                        if let Some(id) = $ids.get(id){
                            $w = $w.$field(Dimension::Of(*id, *v));
                        }        
                    },
                    super::Dimension::KidAreaOf(id, v) => {
                        if let Some(id) = $ids.get(id){
                            $w = $w.$field(Dimension::KidAreaOf(*id, *v));
                        }        
                    },
                }
            },
            _ => (),
        }
    }};
}

/// Implements do_updates for fields shared by all widgets
#[macro_export]
macro_rules! do_updates_common {
    ($self:ident, $w:ident, $ids:ident) => {{
        $w = match &$self.parent {
            Some(id) => {
                match $ids.get(id) {
                    Some(id) => $w.parent(*id),
                    None => $w
                }
            },
            _ => $w,
        };
        
        $w = match &$self.no_parent {
            Some(true) => $w.no_parent(),
            _ => $w,
        };
        
        $w = match &$self.graphics_for {
            Some(id) => {
                match $ids.get(id) {
                    Some(id) => $w.graphics_for(*id),
                    None => $w
                }
            },
            _ => $w,
        };
        
        $w = match &$self.floating {
            Some(b) => $w.floating(*b),
            _ => $w,
        };

        $w = match &$self.crop_kids {
            Some(true) => $w.crop_kids(),
            _ => $w,
        };

        $w = match &$self.scroll_kids {
            Some(true) => $w.scroll_kids(),
            _ => $w,
        };

        $w = match &$self.scroll_kids_vertically {
            Some(true) => $w.scroll_kids_vertically(),
            _ => $w,
        };

        $w = match &$self.scroll_kids_horizontally {
            Some(true) => $w.scroll_kids_horizontally(),
            _ => $w,
        };

        $w = match &$self.place_on_kid_area {
            Some(b) => $w.place_on_kid_area(*b),
            _ => $w,
        };
    }}
}

/// Implements do_updates for fields shared by all Positionable widgets
#[macro_export]
macro_rules! do_updates_position {
    ($self:ident, $w:ident, $ids:ident) => {{
        crate::_do_updates_position_inner!($self, $w, $ids, x_position);
        crate::_do_updates_position_inner!($self, $w, $ids, y_position);

        $w = match &$self.depth {
            Some(v) => $w.depth(*v),
            _ => $w,
        };
    }}
}

/// Implements do_updates for fields shared by all Sizeable widgets
#[macro_export]
macro_rules! do_updates_dimension {
    ($self:ident, $w:ident, $ids:ident) => {{
        crate::_do_updates_dimension_inner!($self, $w, $ids, x_dimension);
        crate::_do_updates_dimension_inner!($self, $w, $ids, y_dimension);
    }}
}


/// Implements do_updates for fields shared by all Colorable widgets
#[macro_export]
macro_rules! do_updates_color {
    ($self:ident, $w:ident, $ids:ident) => {{
        $w = match &$self.color {
            Some(c) => $w.color(conrod_core::Color::Rgba(c.0, c.1, c.2, c.3)),
            _ => $w,
        };
    }}
}