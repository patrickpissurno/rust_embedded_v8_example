use rusty_v8 as v8;
use std::{ fs::File, io::Read, path::PathBuf, error::Error, fmt };

#[derive(Debug, Clone)]
struct JavascriptError{
    msg: String
}

impl JavascriptError {
    fn new(msg: &str) -> JavascriptError {
        JavascriptError{ msg: msg.to_owned() }
    }
}

impl fmt::Display for JavascriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for JavascriptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

//TODO: find a way to capture and forward javascript exceptions back to javascript
pub fn require(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue){
    
    let mut require_impl = || -> Result<(), Box<dyn Error>>{
        if args.length() < 1 || args.length() > 2 || !args.get(0).is_string(){
            return Err(Box::new(JavascriptError::new("invalid function arguments")));
        }
    
        // println!("--------------");
    
        let mut cwd = String::from(".");
        if args.length() > 1{
            let arg = args.get(1);
            if arg.is_string(){
                cwd = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            }
        }
        let cwd = &cwd;
    
        // println!("CWD: {}", cwd);
    
        let fname = args.get(0);
        let fname = fname.to_string(scope).ok_or(JavascriptError::new("invalid function argument (fname)"))?;
        let mut fname = fname.to_rust_string_lossy(scope);
        if !fname.ends_with(".js"){
            fname = format!("{}.js", fname);
        }
    
        let mut path = PathBuf::from(cwd.replace("\\\\?\\", ""));
        path.push(fname);
        let path = path.canonicalize()?;
        let cwd = path.parent().ok_or(JavascriptError::new("fname is not a valid path"))?.to_str().ok_or(JavascriptError::new("fname is not a valid path"))?;
        let path = path.to_str().ok_or(JavascriptError::new("fname is not a valid path"))?;
    
        // println!("CWD: {}", cwd);
        // println!("Path: {}", path);
    
        let mut txt = String::new();
    
        {
            File::open(path)?.read_to_string(&mut txt)?;
        }
    
        let txt = txt.replace("`", "\\`");
        let txt = txt.replace("${", "\\${");
    
        let cwd = cwd.replace("\\", "\\\\");
        let cwd = cwd.replace("'", "\\'");
    
        let module = format!("
            (function(){{
                const module = {{}};
                const _require = require;
                {{
                    const require = (path) => _require(path, '{}');
                    eval(`{}`);
                    return module.exports;
                }}
            }})()
        ", cwd, txt);
    
        // println!("module: {}", module);
    
        let code = v8::String::new(scope, &module).ok_or(JavascriptError::new("failed to allocate module"))?;
        let script = v8::Script::compile(scope, code, None).ok_or(JavascriptError::new("failed to compile required module"))?;
        let result = script.run(scope).ok_or(JavascriptError::new("required module has thrown an error"))?;
    
        rv.set(result);
    
        // rv.set(v8::String::new(scope, &txt).unwrap().into());
        Ok(())
    };

    match require_impl(){
        Ok(()) => (),
        Err(ex) => {
            let err = v8::String::new(scope, &ex.to_string()).unwrap();
            let err = v8::Exception::error(scope, err);
            scope.throw_exception(err);
        }
    }
}