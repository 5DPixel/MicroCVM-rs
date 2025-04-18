mod cpu;
mod disk;
mod render;
mod screen;
mod types;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let mut vcpu = cpu::MicroCVMCpu::empty();
    let mut vdisk = disk::MicroCVMDisk::empty();

    match vcpu.read_memory_from_file("../../examples/draw_line.bin") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error reading binary: {}", e);
        }
    }
    // vcpu.memory[3] = cpu::OpcodeType::Mov as u16;
    // vcpu.memory[4] = 255;
    // vcpu.memory[5] = cpu::Register::V1 as u16;
    // vcpu.memory[6] = cpu::OpcodeType::Hlt as u16;
    // vcpu.memory[3] = cpu::OpcodeType::Hlt as u16;
    //println!("{:?}", vcpu.memory);

    loop {
        let opcode: u16 = vcpu.memory[vcpu.pc as usize];
        if opcode == cpu::OpcodeType::Hlt as u16 {
            break;
        }
        let opcode_length = vcpu.execute_instruction();

        vcpu.pc += opcode_length;
    }

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = render::App::new(
        vcpu.framebuffer_width as u32,
        vcpu.framebuffer_height as u32,
        vcpu.video_memory,
    );
    let _ = event_loop.run_app(&mut app);
}
