extern crate conrod_core;
extern crate conrod_glium;
extern crate conrod_winit;
extern crate glium;
use conrod_core::widget;
use glium::Surface;
use rusty_v8 as v8;
use std::collections::HashMap;
mod js;
mod js_widgets;
mod support;
use js::{run_script, run_script_no_result};
use js_widgets::JsWidget;

fn main() {
    js::init();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    js::set_globals(&context, scope);

    let js_gc = precompile_script!(scope, "gc()");

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("V8 + Conrod")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();
    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    let mut gen = ui.widget_id_generator();
    let mut ids: HashMap<String, widget::Id> = HashMap::new();

    eval_script!(scope, "let screen1 = require('./js/screen1')");

    // executes the setup function of the current screen in order to generate
    // the ids that conrod requires
    {
        let ids_as_string: Vec<String> = eval_script_to_object!(scope, "screen1.setup()");
        for key in ids_as_string {
            let id = gen.next();
            ids.insert(key, id);
        }
    }

    let screen1_draw = precompile_script_to_object!(scope, "screen1.draw()");

    run_script_no_result(scope, js_gc);

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
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

        // executes the draw function of the current screen
        let widgets: Vec<JsWidget> = run_script_to_object!(scope, screen1_draw);

        {
            let ui = &mut ui.set_widgets();

            *ids.entry(String::from("window")).or_insert(ui.window) = ui.window;

            for widget in widgets {
                widget.do_updates(ui, &ids);
            }

            // calls the V8 Garbage Collector to avoid leaking memory
            //TODO: this should not be called this frequently
            run_script_no_result(scope, js_gc);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
