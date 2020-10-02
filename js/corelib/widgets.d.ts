/*
    This file provides typing for the UI widgets. Those are
    constructed to be as similar as possible to conrod's Rust crate.

    However, here there are no method calls. This is a purely
    declarative way of defining the user interface.

    Sometimes it might be worth it to take a look at conrod's docs
    for understanding how it works and what to expect.

    https://docs.rs/conrod_core/0.70.0/conrod_core/
*/

type Color = [ red: number, green: number, blue: number, alpha: number ];

export namespace Position
{
    interface AbsolutePosition {
        Absolute: number
    }

    interface RelativePosition {
        Relative: [ ScalarWrapper|AlignWrapper|DirectionWrapper|PlaceWrapper, string|null ]
    }

    interface ScalarWrapper {
        Scalar: number
    }

    interface AlignWrapper {
        Align: AlignEnum
    }

    enum AlignEnum {
        Start = 'Start',
        Middle = 'Middle',
        End = 'End'
    }

    interface DirectionWrapper {
        Direction: [ DirectionEnum, number ]
    }

    enum DirectionEnum {
        Forwards = 'Forwards',
        Backwards = 'Backwards',
    }

    interface PlaceWrapper {
        Place: StartWrapper|string|EndWrapper
    }

    interface StartWrapper {
        Start?: number
    }

    interface EndWrapper {
        End?: number
    }

    enum PlaceEnum {
        Start = 'Start',
        Middle = 'Middle',
        End = 'End'
    }

    export function Absolute(value: number): AbsolutePosition;
    export function Relative(kind: ScalarWrapper|AlignWrapper|DirectionWrapper|PlaceWrapper, id?: string): RelativePosition;
    export function Scalar(value: number): ScalarWrapper;
    export function Align(kind: AlignEnum): AlignWrapper;
    export function Direction(direction: DirectionEnum, scalar: number): DirectionWrapper;
    export function Place(kind: PlaceEnum, margin?: number): PlaceWrapper;
}

export namespace Dimension
{
    interface AbsoluteDimension {
        Absolute: number
    }

    interface OfDimension {
        Of: [ string, number|null ]
    }

    interface KidAreaOfDimension {
        KidAreaOf: [ string, number|null ]
    }

    export function Absolute(value: number): AbsoluteDimension;
    export function Of(id: string, padding?: number): OfDimension;
    export function KidAreaOf(id: string, padding?: number): KidAreaOfDimension;
}

// shared by all widgets
// https://docs.rs/conrod_core/0.70.0/conrod_core/widget/trait.Widget.html
interface Widget {
    id: string,
    parent?: string,
    no_parent?: boolean,
    graphics_for?: string,
    floating?: boolean,
    crop_kids?: boolean,
    scroll_kids?: boolean,
    scroll_kids_vertically?: boolean,
    scroll_kids_horizontally?: boolean,
    place_on_kid_area?: boolean,
}

// colorable trait
// https://docs.rs/conrod_core/0.70.0/conrod_core/color/trait.Colorable.html
interface Colorable {
    color?: Color,
}

// positionable trait
// https://docs.rs/conrod_core/0.70.0/conrod_core/position/trait.Positionable.html
interface Positionable {
    depth?: number,
    x_position?: Position.AbsolutePosition | Position.RelativePosition,
    y_position?: Position.AbsolutePosition | Position.RelativePosition,
}

// sizeable trait
// https://docs.rs/conrod_core/0.70.0/conrod_core/position/trait.Sizeable.html
interface Sizeable {
    x_dimension?: Dimension.AbsoluteDimension | Dimension.OfDimension | Dimension.KidAreaOfDimension,
    y_dimension?: Dimension.AbsoluteDimension | Dimension.OfDimension | Dimension.KidAreaOfDimension,
}

// Text widget
interface TextWidgetWrapper {
    Text: TextWidget
}

interface TextWidget extends Widget, Colorable, Positionable, Sizeable {
    text?: string,
    font_size?: number,
    left_justify?: boolean,
    center_justify?: boolean,
    right_justify?: boolean,
    line_spacing?: number,
    no_line_wrap?: boolean,
    wrap_by_word?: boolean,
    wrap_by_character?: boolean,
}

// Rectangle widget
interface RectangleWidgetWrapper {
    Rectangle: RectangleWidget
}

interface RectangleWidget extends Widget, Colorable, Positionable, Sizeable {
    fill?: boolean,
    outline?: boolean,
}

// Button widget
interface ButtonWidgetWrapper {
    Button: ButtonWidget
}

interface ButtonWidget extends Widget, Colorable, Positionable, Sizeable {
    enabled?: boolean,
    hover_color?: Color,
    press_color?: Color,
    label?: string,
}

/**
 * https://docs.rs/conrod_core/0.70.0/conrod_core/widget/primitive/text/struct.Text.html
 */
export function Text(id: string, params?: TextWidget): TextWidgetWrapper;

/**
 * https://docs.rs/conrod_core/0.70.0/conrod_core/widget/primitive/shape/rectangle/struct.Rectangle.html
 */
export function Rectangle(id: string, params?: RectangleWidget): RectangleWidgetWrapper;

/**
 * https://docs.rs/conrod_core/0.70.0/conrod_core/widget/button/struct.Button.html
 */
export function Button(id: string, params?: ButtonWidget): ButtonWidgetWrapper;