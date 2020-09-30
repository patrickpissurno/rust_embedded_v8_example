/// <reference path="./corelib/global.d.ts"/>

const { Text, Position, Dimension } = require('./corelib/widgets');
const { Rgb, COLORS } = require('./corelib/utils');

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

        this.color_mode = 'g+';
        this.color_speed = 0.1;
        this.r = 1;
        this.g = 0;
        this.b = 0;

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

        switch(this.color_mode){
            case 'g+':
                if(this.g < 1)
                    this.g += this.color_speed;
                if(this.g >= 1)
                    this.color_mode = 'r-';
                break;
            case 'r-':
                if(this.r > 0)
                    this.r -= this.color_speed;
                if(this.r <= 0)
                    this.color_mode = 'b+';
                break;
            case 'b+':
                if(this.b < 1)
                    this.b += this.color_speed;
                if(this.b >= 1)
                    this.color_mode = 'g-';
                break;
            case 'g-':
                if(this.g > 0)
                    this.g -= this.color_speed;
                if(this.g <= 0)
                    this.color_mode = 'r+';
                break;
            case 'r+':
                if(this.r < 1)
                    this.r += this.color_speed;
                if(this.r >= 1)
                    this.color_mode = 'b-';
                break;
            case 'b-':
                if(this.b > 0)
                    this.b -= this.color_speed;
                if(this.b <= 0)
                    this.color_mode = 'g+';
                break;
        }
        this.r = this.r > 1 ? 1 : (this.r < 0 ? 0 : this.r);
        this.g = this.g > 1 ? 1 : (this.g < 0 ? 0 : this.g);
        this.b = this.b > 1 ? 1 : (this.b < 0 ? 0 : this.b);
    
        // log(this.text);

        let xpos = Position.Relative(Position.Direction(Position.DirectionEnum.Forwards, 60), ID.text);
    
        return [
            Text(ID.text, { text: this.text, font_size: 32, color: Rgb(this.r, this.g, this.b) }),
            Text(ID.text2, { text: '123', font_size: this.size, color: COLORS.WHITE, x_position: xpos, x_dimension: Dimension.Absolute(10) }),
        ];
    }

}

module.exports = new Screen1();