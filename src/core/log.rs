use rusty_v8 as v8;

pub fn log(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue){
    if args.length() > 0 {
        let data = args.get(0);
        let data = data.to_string(scope).unwrap();
        let data = data.to_rust_string_lossy(scope);
        println!("[JS] {}", data);
    }
}