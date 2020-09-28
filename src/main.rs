#[macro_use] extern crate conrod;
extern crate find_folder;
use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Positionable, Colorable, Widget};
use rusty_v8 as v8;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

mod core;

macro_rules! run_script_to_object {
    ($scope:expr, $script:expr) => {{
        let script = format!("JSON.stringify({})", $script);
        let res = run_script($scope, &script);

        serde_json::from_str(&res).expect("Failed to deserialize JSON")
    }};
}

#[derive(Deserialize)]
struct Text {
    id: String,
    text: String,
}

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    set_globals(&context, scope);

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
                    .with_title("V8 + Conrod")
                    .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
                    .with_vsync(true)
                    .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut gen = ui.widget_id_generator();
    let mut ids: HashMap<String, widget::Id> = HashMap::new();

    run_script(scope, "let screen1 = require('./js/screen1')");

    {
        let ids_as_string: Vec<String> = run_script_to_object!(scope, "screen1.setup()");
        for key in ids_as_string {
            let id = gen.next();
            ids.insert(key, id);
        }
    }

    'main: loop {
        {
            let ui = &mut ui.set_widgets();

            let txts: Vec<Text> = run_script_to_object!(scope, "screen1.draw()");

            for txt in txts{
                let id = ids.get(&txt.id).unwrap();

                widget::Text::new(&txt.text)
                    .middle_of(ui.window)
                    .color(conrod::color::WHITE)
                    .font_size(32)
                    .set(*id, ui);
            }
        }
            
        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        for event in events{
            match event {
                glium::glutin::Event::WindowEvent { event, ..} => match event {
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }
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

fn run_script(scope: &mut v8::ContextScope<v8::HandleScope>, script: &str) -> String {
    let code = v8::String::new(scope, script).unwrap();
    //println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).expect("Uncaught exception while executing the script");
    let result = result.to_string(scope).unwrap();

    //println!("result: {}\n", result.to_rust_string_lossy(scope));
    result.to_rust_string_lossy(scope)
}