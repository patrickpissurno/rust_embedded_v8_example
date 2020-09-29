const COLORS = {
    RED: Rgb(1, 0, 0),
    GREEN: Rgb(0, 1, 0),
    BLUE: Rgb(0, 0, 1),
    BLACK: Rgb(0, 0, 0),
    WHITE: Rgb(1, 1, 1),
};

module.exports = {
    COLORS,
    Rgb
};

function Rgb(red, green, blue, alpha = 1){
    return [ red, green, blue, alpha ];
}