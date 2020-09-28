const { hello, world } = require('./other');
const strcomb = require('./utils/strcomb');
log(strcomb(hello, ' ', world));