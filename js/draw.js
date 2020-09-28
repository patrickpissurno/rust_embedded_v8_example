module.exports = draw;

let text = '';
let mode = '+';
let target = 'Hello World!';
function draw(){
    if(mode === '+' && text !== target)
        text = text + target[text.length];
    else if(mode === '+' && text === target)
        mode = '-';
    else if(mode === '-' && text.length > 0)
        text = text.substr(0, text.length - 1);
    else if(mode === '-' && text.length === 0)
        mode = '+';

    log(text);

    return text;
}