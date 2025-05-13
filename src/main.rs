//#![windows_subsystem = "windows"]

mod cpu;
mod disk;
mod render;
mod screen;
mod types;

use std::sync::{Arc, Mutex};
use std::thread;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::cpu::Register;

fn main() {
    let vcpu = Arc::new(Mutex::new(cpu::MicroCVMCpu::empty()));

    {
        let mut vcpu_locked = vcpu.lock().unwrap();
        if let Err(e) = vcpu_locked.read_memory_from_file("examples/string.bin") {
            eprintln!("error reading binary: {}", e);
        }
    }

    let vcpu_for_cpu_thread = Arc::clone(&vcpu);
    thread::spawn(move || {
        loop {
            let mut vcpu = vcpu_for_cpu_thread.lock().unwrap();
            let current_pc = vcpu.pc;
            let opcode_word = vcpu.memory[vcpu.pc as usize];

            if opcode_word == cpu::OpcodeType::Hlt as u16 {
                break;
            }

            let opcode_length = vcpu.execute_instruction();

            if vcpu.pc == current_pc {
                vcpu.pc += opcode_length;
            }
        }

        println!("Memory execution finished.");
    });

    let (framebuffer_width, framebuffer_height) = {
        let vcpu_locked = vcpu.lock().unwrap();
        (
            vcpu_locked.framebuffer_width,
            vcpu_locked.framebuffer_height,
        )
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = render::App::new(framebuffer_width as u32, framebuffer_height as u32, vcpu);
    let _ = event_loop.run_app(&mut app);

}
