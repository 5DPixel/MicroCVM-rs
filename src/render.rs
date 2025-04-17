use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalPosition;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    width: u32,
    height: u32,
    video_memory: Vec<super::types::Color>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_position(LogicalPosition::new(0, 0))
            .with_title("Virtual Machine Window");

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window.clone());

        let surface_texture = SurfaceTexture::new(self.width, self.height, window.clone());
        let pixels = Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl App {
    fn render(&mut self) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();

            if self.video_memory.len() < frame.len() {
                eprintln!(
                    "Error: Video memory size does not match framebuffer size. Frame size: {}, Video memory size: {}",
                    frame.len(),
                    self.video_memory.len()
                );
                return;
            }

            let mut byte_index = 0;
            for color in &self.video_memory {
                if !(byte_index + 3 < frame.len()) {
                    break;
                }
                frame[byte_index] = color.r;
                frame[byte_index + 1] = color.g;
                frame[byte_index + 2] = color.b;
                frame[byte_index + 3] = color.a;
                byte_index += 4;
            }

            pixels.render().unwrap();
        }
    }

    pub fn new(width: u32, height: u32, video_memory: Vec<super::types::Color>) -> Self {
        Self {
            window: None,
            pixels: None,
            width,
            height,
            video_memory,
        }
    }
}
