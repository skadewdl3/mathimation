use std::sync::Arc;

use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::state::State;


pub struct Application {
    pub window: Arc<Window>,
    pub state: State<'static>,
    // Does not actually pause the event loop. Just says whether update function will run or not.
    pub paused: bool,
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
        crate::log("updating");
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


