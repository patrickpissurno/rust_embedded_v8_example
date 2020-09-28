module.exports = {
    Text
};

function Text(id, params){
    return { Text: Object.assign({ id }, params) };
}