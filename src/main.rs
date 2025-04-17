mod cpu;
mod disk;
mod render;
mod types;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let mut vcpu = cpu::MicroCVMCpu::empty();
    let vdisk = disk::MicroCVMDisk::empty();

    vcpu.memory[0] = cpu::OpcodeType::Mov as u8;
    vcpu.memory[1] = cpu::Register::R0 as u8;
    vcpu.memory[2] = 16;
    vcpu.memory[3] = cpu::OpcodeType::Hlt as u8;
    // match vcpu.read_memory_from_file("../../tests/mov.bin") {
    //     Ok(_) => {}
    //     Err(e) => {
    //         eprintln!("error reading binary: {}", e);
    //     }
    // }

    loop {
        let opcode: u8 = vcpu.memory[vcpu.pc as usize];
        if opcode == cpu::OpcodeType::Hlt as u8 {
            break;
        }
        vcpu.execute_instruction();

        vcpu.pc += 1;
    }

    println!("{}", vcpu.registers[cpu::Register::R0 as usize]);

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    // for i in 0..2000 {
    //     vcpu.video_memory[i].r = 255;
    //     vcpu.video_memory[i].g = 0;
    //     vcpu.video_memory[i].b = 255;
    // }

    let mut app = render::App::new(768, 576, vcpu.video_memory);
    let _ = event_loop.run_app(&mut app);

    println!("{}", vcpu.registers[cpu::Register::R0 as usize]);
}
