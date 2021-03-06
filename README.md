# Embedding V8 (JavaScript engine) into a Rust program
This a research project about how to properly embed V8 into a Rust program.

I'm using `rusty_v8`, which does all the heavy lifting. But what you get from it is bare-bones access to V8.
To do anything useful you'll probably have to provide a way to read files, require modules, etc.

I'm not trying to recreate Node.js, Deno or anything like that here. This is about correctly embedding V8, so you can write part of your Rust applications in JavaScript.

### This branch contains my experimenal UI integration work
It's building on top of the `legacy` branch and further refining it based on a "real world" use-case: driving UI from JavaScript. It uses `conrod` with `glium` for handling OpenGL and drawing stuff. Actually, `conrod` also supports `vulkano` and once I get a solid foundation, the goal is migrating it to `vulkano`.

### What I already got working:
- a way to call JavaScript code from Rust passing data and getting the result back
- a way to call Rust code from JavaScript passing data and getting the result back
- `log(msg)`, my poor-man's implementation of the classic `console.log(msg)`
- `require`, my poor-man's implementation of something that resembles Node.js' module system, except mine has a bunch of limitations and uses `eval`. But it kinda works. Be advised that it only supports absolute paths (eg. `require('C:\\files\\main')`) and relative paths (eg. `require('./main')`). It does not support those fancy `require('modulename')` calls. It also doesn't [de-dupe (cache) modules](https://nodejs.org/api/modules.html#modules_caching) yet. You can use both foward slashes and back slashes to specify your paths, but do not mix them. Finally, you can optionally specify the extension (as of now, if you do specify, it must end in '.js').
- basic rendering
- a generic way of defining (creating) UI elements from JavaScript
- a generic way to update UI elements from JavaScript
- a generic abstraction that allows `conrod` widgets to be "bridged" in order to be available within JavaScript
- `conrod`'s `Text` widget (a bunch of properties are already available and working, with more being very easy to add)

### What is missing:
- Everything not mentioned above
- I did implement some sort of error handling for my `require` implementation, but it should be noted that it won't forward the stack-trace back to JavaScript. This means that you can wrap a `require` call with a try-catch statement, and if the module throws an exception, it will be caught. However you won't get the stack-trace, just a miserable `required module has thrown an error` `Error` object. I'd like to make the `Error` object contain the stack-trace, but I still need to figure out a way to do it.
- Proper way of letting V8 know that it can dispose some unused RAM. The GC is doing it's job for the most part, but there are some small memory leaks. This issue was worse before, and I already got some success in reducing the leak size. I've opened an issue on the `rusty_v8` repo, because after trying my best there still are leaks. (Albeit small, after a long period of execution, those add up).

### What this can be useful for
- Use modern JavaScript as an embedded scripting language, instead of using LuaJIT (which is limited to Lua 5.1). For instance, you could write games and applications mainly in Rust, but some parts that you'd like to be easily moddable (eg. UI stuff) could be written in JavaScript.
- Probably many other use cases

### Why am I doing this
- As a proof-of-concept, to see if my idea of mixing Rust and JavaScript to create easily moddable applications would be a good one
- To learn Rust

PS: I'm still learning Rust, so any feedback will be very much appreciated.

### How to run
- I think I'm using Rust stable, so it shouldn't require nightly
- Clone the repo and run `cargo run`
- A window should pop up with some animated graphics
- I'm running Windows 10 x64, and have not tested anything under Linux or macOS
- If you find any issues, please report them

### License
`rusty_v8` and `V8` itself are licensed under their own licenses. The following license is only applicable to the source-code contained in this repo.

```
MIT License

Copyright (c) 2020 Patrick Pissurno

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
