type Color = [ red: number, green: number, blue: number, alpha: number ];

/**
 * Creates a RGB(A) color
 * @param red float from 0 to 1
 * @param green float from 0 to 1
 * @param blue float from 0 to 1
 * @param alpha float from 0 to 1
 */
export function Rgb(red: number, green: number, blue: number, alpha?: number): Color;