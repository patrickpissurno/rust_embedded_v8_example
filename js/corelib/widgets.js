const Position = {
    Absolute(value){
        return { Absolute: value };
    },
    Relative(kind, id){
        return { Relative: [ kind, id ] };
    },
    Scalar(value){
        return { Scalar: value };
    },
    Align(kind){
        return { Align: kind };
    },
    AlignEnum: {
        Start: 'Start',
        Middle: 'Middle',
        End: 'End'
    },
    Direction(direction, scalar){
        return { Direction: [ direction, scalar ] };
    },
    DirectionEnum: {
        Forwards: 'Forwards',
        Backwards: 'Backwards',
    },
    Place(kind, margin){
        if(kind === this.PlaceEnum.Middle)
            return { Place: kind };
        return { Place: { [kind]: margin || 0 } };
    },
    PlaceEnum: {
        Start: 'Start',
        Middle: 'Middle',
        End: 'End'
    }
};

module.exports = {
    Position,
    Text,
};

function Text(id, params){
    return { Text: Object.assign({ id }, params) };
}