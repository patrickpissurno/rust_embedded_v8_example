const Position = {
    Absolute(value){
        return { Absolute: value };
    }
};

module.exports = {
    Position,
    Text,
};

function Text(id, params){
    return { Text: Object.assign({ id }, params) };
}