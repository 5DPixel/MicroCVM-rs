use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::cpu;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    width: u32,
    height: u32,
    cpu: Arc<Mutex<cpu::MicroCVMCpu>>,
}

fn physical_key_to_keycode(key: &PhysicalKey) -> u16 {
    match key {
        PhysicalKey::Code(code) => match code {
            winit::keyboard::KeyCode::KeyA => 0x41,
            winit::keyboard::KeyCode::KeyB => 0x42,
            winit::keyboard::KeyCode::Enter => 0x0D,
            winit::keyboard::KeyCode::ArrowUp => 0x80,
            winit::keyboard::KeyCode::ArrowDown => 0x81,
            winit::keyboard::KeyCode::Space => 0x20,
            _ => 0x00,
        },
        _ => 0x00,
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_position(LogicalPosition::new(0, 0))
            .with_title("Virtual Machine Window");

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window.clone());

        let surface_texture = SurfaceTexture::new(self.width / 2, self.height / 2, window.clone());
        let pixels = Pixels::new(self.width / 2, self.height / 2, surface_texture).unwrap();

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
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let physical_key = event.physical_key;
                let mut cpu = self.cpu.lock().unwrap();
                if event.state.is_pressed() {
                    cpu.registers[cpu::Register::index(cpu::Register::K0) as usize] =
                        physical_key_to_keycode(&physical_key);
                } else {
                    cpu.registers[cpu::Register::index(cpu::Register::K0) as usize] = 0x5F;
                }
            }
            _ => (),
        }
    }
}

impl App {
    fn render(&mut self) {
        let cpu = self.cpu.lock().unwrap();

        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();

            if cpu.video_memory.len() < frame.len() {
                eprintln!(
                    "Error: Video memory size does not match framebuffer size. Frame size: {}, Video memory size: {}",
                    frame.len(),
                    cpu.video_memory.len()
                );
                return;
            }

            let mut byte_index = 0;
            for color in &cpu.video_memory {
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

    pub fn new(width: u32, height: u32, cpu: Arc<Mutex<cpu::MicroCVMCpu>>) -> Self {
        Self {
            window: None,
            pixels: None,
            width,
            height,
            cpu,
        }
    }
}
