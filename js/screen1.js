/// <reference path="./corelib/global.d.ts"/>

const { Text } = require('./corelib/widgets');

const ID = {
    text: 'text',
    text2: 'text2'
};

class Screen1 {

    setup(){
        this.text = '';
        this.text_mode = '+';
        this.text_target = 'Hello World!';

        this.size_mode = '+';
        this.size_min = 18;
        this.size_max = 48;
        this.size = this.size_min;

        return Object.keys(ID);
    }

    draw(){
        if(this.text_mode === '+' && this.text !== this.text_target)
            this.text = this.text + this.text_target[this.text.length];
        else if(this.text_mode === '+' && this.text === this.text_target)
            this.text_mode = '-';
        else if(this.text_mode === '-' && this.text.length > 0)
            this.text = this.text.substr(0, this.text.length - 1);
        else if(this.text_mode === '-' && this.text.length === 0)
            this.text_mode = '+';

        if(this.size_mode === '+' && this.size < this.size_max)
            this.size += 1;
        else if(this.size_mode === '+' && this.size >= this.size_max)
            this.size_mode = '-';
        else if(this.size_mode === '-' && this.size > this.size_min)
            this.size -= 1;
        else if(this.size_mode === '-' && this.size <= this.size_min)
            this.size_mode = '+';
    
        log(this.text);
    
        return [
            Text(ID.text, { text: this.text }),
            Text(ID.text2, { text: '123', font_size: this.size }),
        ];
    }

}

module.exports = new Screen1();