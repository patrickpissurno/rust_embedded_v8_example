/*
    This file provides typing for the UI widgets. Those are
    constructed to be as similar as possible to conrod's Rust crate.

    However, here there are no method calls. This is a purely
    declarative way of defining the user interface.

    Sometimes it might be worth it to take a look at conrod's docs
    for understanding how it works and what to expect.

    https://docs.rs/conrod_core/0.70.0/conrod_core/
*/

interface TextWidgetWrapper {
    Text: TextWidget
}

interface TextWidget {
    id: string,
    text: string?,
    font_size: number?
}

/**
 * https://docs.rs/conrod_core/0.70.0/conrod_core/widget/primitive/text/struct.Text.html
 */
export function Text(id: string, params: TextWidget): TextWidgetWrapper;