use rusty_v8 as v8;
use std::env;

mod core;

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    set_globals(&context, scope);
    // run_script(scope, "log(123)");
    // run_script(scope, "get42()");
    // run_script(scope, "require('./example/main')");

    let mut args = env::args();
    if args.len() > 1{
        args.next();
        let fname = &args.next().unwrap();
        run_script(scope, &format!("require('{}')", fname));
    }
}

fn set_globals(context: &v8::Local<v8::Context>, scope: &mut v8::ContextScope<v8::HandleScope>){
    let global = context.global(scope);

    macro_rules! set_func {
        ($name:expr, $callback:expr) => {{
            let fn_template = v8::FunctionTemplate::new(scope, $callback);
        
            let func = fn_template
                .get_function(scope)
                .expect("Unable to create function");
    
            let key = v8::String::new(scope, $name).unwrap();
            global.set(scope, key.into(), func.into());
        }};
    }

    set_func!("log", core::log::log);
    set_func!("require", core::require::require);

    set_func!("get42", |scope: &mut v8::HandleScope, _args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue|{
        rv.set(v8::Integer::new(scope, 42).into());
    });
}

fn run_script(scope: &mut v8::ContextScope<v8::HandleScope>, script: &str){
    let code = v8::String::new(scope, script).unwrap();
    //println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).expect("Uncaught exception while executing the script");
    let _result = result.to_string(scope).unwrap();
    //println!("result: {}\n", result.to_rust_string_lossy(scope));
}