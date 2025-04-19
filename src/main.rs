mod cpu;
mod disk;
mod render;
mod screen;
mod types;

use cpu::{FLAG_ZERO, Register};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let mut vcpu = cpu::MicroCVMCpu::empty();
    //let mut vdisk = disk::MicroCVMDisk::empty();

    match vcpu.read_memory_from_file("../../examples/jmp.bin") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error reading binary: {}", e);
        }
    }

    loop {
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

    println!(
        "Finished, r0 value: {}",
        vcpu.registers[Register::index(Register::R0)]
    );

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = render::App::new(
        vcpu.framebuffer_width as u32,
        vcpu.framebuffer_height as u32,
        vcpu.video_memory,
    );
    let _ = event_loop.run_app(&mut app);
}
