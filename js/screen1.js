class Screen1 {

    setup(){
        this.text = '';
        this.mode = '+';
        this.target = 'Hello World!';

        return 'text,text2';
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
    
        return `text=${this.text},text2=123`;
    }

}

module.exports = new Screen1();