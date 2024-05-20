use std::cell::RefCell;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

struct Application {
    window: Arc<Window>,
    state: State<'static>,
    // Does not actually pause the event loop. Just says whether update function will run or not.
    paused: bool,
}

thread_local! {
pub static APP: RefCell<Option<Application>> = RefCell::new(None);
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>, size: PhysicalSize<u32>) -> State<'a> {
        // Create a new instance and adapter
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = {
            instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap()
        };

        // Request a device and queue
        let (device, queue) = {
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::empty(),
                        // WebGL doesn't support all of wgpu's features, so if
                        // we're building for the web, we'll have to disable some.
                        required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                        label: None,
                    },
                    None, // Trace path
                )
                .await
                .unwrap()
        };

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Application {
    pub async fn new(window: Arc<Window>, size: PhysicalSize<u32>) -> Application {
        let state = State::new(window.clone(), size).await;
        Self {
            window,
            state,
            paused: true,
        }
    }
    pub fn set_paused (&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn update(&self) {
        let _state = &self.state;
        log("updating");
    }

    pub fn render(&self) {
        let state = &self.state;
        let output = state.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        todo!()
    }
    pub fn input(&self, event: &WindowEvent) {
        todo!()
    }
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

