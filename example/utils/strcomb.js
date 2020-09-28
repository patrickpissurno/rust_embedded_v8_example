const exclamation = require('./exclamation');

module.exports = function(...args){
    return exclamation(args.join(''));
}