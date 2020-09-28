const ID = {
    text: 'text',
    text2: 'text2'
};

class Screen1 {

    setup(){
        this.text = '';
        this.mode = '+';
        this.target = 'Hello World!';

        return Object.keys(ID);
    }

    draw(){
        if(this.mode === '+' && this.text !== this.target)
            this.text = this.text + this.target[this.text.length];
        else if(this.mode === '+' && this.text === this.target)
            this.mode = '-';
        else if(this.mode === '-' && this.text.length > 0)
            this.text = this.text.substr(0, this.text.length - 1);
        else if(this.mode === '-' && this.text.length === 0)
            this.mode = '+';
    
        log(this.text);
    
        return [
            { Text: { id: ID.text, text: this.text } },
            { Text: { id: ID.text2, text: '123' } },
        ];
    }

}

module.exports = new Screen1();