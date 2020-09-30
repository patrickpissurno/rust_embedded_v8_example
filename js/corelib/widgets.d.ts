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

interface TextWidgetWrapper {
    Text: TextWidget
}

interface TextWidget {
    id: string,
    text?: string,
    font_size?: number,
    color?: Color,
    x_position?: Position.AbsolutePosition,
    y_position?: Position.AbsolutePosition,
    left_justify?: boolean,
    center_justify?: boolean,
    right_justify?: boolean,
    line_spacing?: number,
    no_line_wrap?: boolean,
    wrap_by_word?: boolean,
    wrap_by_character?: boolean,
}

/**
 * https://docs.rs/conrod_core/0.70.0/conrod_core/widget/primitive/text/struct.Text.html
 */
export function Text(id: string, params?: TextWidget): TextWidgetWrapper;