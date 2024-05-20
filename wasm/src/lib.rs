mod app;
mod state;

use std::cell::RefCell;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use app::Application;

thread_local! {
    pub static APP: RefCell<Option<Application>> = RefCell::new(None);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub async fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    // it's deprecated, but I couldn't find another way
    let window = event_loop
        .create_window(Window::default_attributes())
        .unwrap();

    let window = Arc::new(window);

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| {
                window.set_min_inner_size(Some(PhysicalSize::new(450, 400)));
                win.document()
            })
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let app = Application::new(window.clone(), PhysicalSize::new(800, 400)).await;

    APP.with(|a| {
        *a.borrow_mut() = Some(app);
    });

    let _ = game_loop::game_loop(
        event_loop,
        window,
        -1,
        60,
        0.1,
        |_| {
            APP.with(|app| {
                if app.borrow().as_ref().unwrap().paused { return }
                app.borrow().as_ref().unwrap().update();
            })
        }, // update
        |_| {
            APP.with(|app| {
                if app.borrow().as_ref().unwrap().paused { return }
                app.borrow().as_ref().unwrap().render();
            })
        }, // render
        |_, _| {}, // events
    );
}

#[wasm_bindgen]
pub fn run() {
    APP.with(|a| {
        a.borrow_mut().as_mut().unwrap().set_paused(false);
    })
}

#[wasm_bindgen]
pub fn pause(){
    APP.with(|a| {
        a.borrow_mut().as_mut().unwrap().set_paused(true);
    })
}

