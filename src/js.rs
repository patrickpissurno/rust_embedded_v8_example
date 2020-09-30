use rusty_v8 as v8;

pub mod log;
pub mod require;

pub fn run_script(
    scope: &mut v8::ContextScope<v8::HandleScope>,
    script: v8::Local<v8::Script>,
) -> String {
    let result = script
        .run(scope)
        .expect("Uncaught exception while executing the script");

    let result = result.to_string(scope).unwrap();

    //println!("result: {}\n", result.to_rust_string_lossy(scope));
    result.to_rust_string_lossy(scope)
}

pub fn run_script_no_result(
    scope: &mut v8::ContextScope<v8::HandleScope>,
    script: v8::Local<v8::Script>,
) {
    script
        .run(scope)
        .expect("Uncaught exception while executing the script");
}

#[macro_export]
macro_rules! precompile_script {
    ($scope:expr, $script:expr) => {{
        let code = v8::String::new($scope, $script).unwrap();
        //println!("javascript code: {}", code.to_rust_string_lossy(scope));

        v8::Script::compile($scope, code, None).unwrap()
    }};
}

#[macro_export]
macro_rules! precompile_script_to_object {
    ($scope:expr, $script:expr) => {{
        let script = format!("JSON.stringify({})", $script);
        precompile_script!($scope, &script)
    }};
}

#[macro_export]
macro_rules! run_script_to_object {
    ($scope:expr, $script:expr) => {{
        let res = run_script($scope, $script);
        serde_json::from_str(&res).expect("Failed to deserialize JSON")
    }};
}

#[macro_export]
macro_rules! eval_script {
    ($scope:expr, $script:expr) => {{
        let script = precompile_script!($scope, $script);
        run_script($scope, script)
    }};
}

#[macro_export]
macro_rules! eval_script_to_object {
    ($scope:expr, $script:expr) => {{
        let script = precompile_script_to_object!($scope, $script);
        run_script_to_object!($scope, script)
    }};
}

pub fn init() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    v8::V8::set_flags_from_command_line(vec!["".to_string(), "--expose-gc".to_string()]);
}

pub fn set_globals(
    context: &v8::Local<v8::Context>,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) {
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

    set_func!("log", log::log);
    set_func!("require", require::require);

    set_func!("get42", |scope: &mut v8::HandleScope,
                        _args: v8::FunctionCallbackArguments,
                        mut rv: v8::ReturnValue| {
        rv.set(v8::Integer::new(scope, 42).into());
    });
}
