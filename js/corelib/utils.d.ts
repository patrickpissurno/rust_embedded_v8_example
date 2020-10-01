type Color = [ red: number, green: number, blue: number, alpha: number ];

/**
 * Creates a RGB(A) color
 * @param red float from 0 to 1
 * @param green float from 0 to 1
 * @param blue float from 0 to 1
 * @param alpha float from 0 to 1
 */
export function Rgb(red: number, green: number, blue: number, alpha?: number): Color;

export const COLORS = {
    RED: Rgb(1, 0, 0),
    GREEN: Rgb(0, 1, 0),
    BLUE: Rgb(0, 0, 1),
    BLACK: Rgb(0, 0, 0),
    WHITE: Rgb(1, 1, 1),
};

export const DEFAULT_IDS = {
    window: 'window'
};