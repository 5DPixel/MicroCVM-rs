mod cpu;
mod disk;
mod render;
mod screen;
mod types;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let mut vcpu = cpu::MicroCVMCpu::empty();
    let mut vdisk = disk::MicroCVMDisk::empty();

    // vcpu.memory[0] = cpu::OpcodeType::Mov as u8;
    // vcpu.memory[1] = cpu::Register::V0 as u8;
    // vcpu.memory[2] = 255;
    // vcpu.memory[3] = cpu::OpcodeType::Mov as u8;
    // vcpu.memory[4] = cpu::Register::V1 as u8;
    // vcpu.memory[5] = 0;
    // vcpu.memory[6] = cpu::OpcodeType::Mov as u8;
    // vcpu.memory[7] = cpu::Register::V2 as u8;
    // vcpu.memory[8] = 255;
    // vcpu.memory[9] = cpu::OpcodeType::Call as u8;
    // vcpu.memory[10] = cpu::FunctionCall::FillRect as u8;
    // vcpu.memory[11] = cpu::OpcodeType::Hlt as u8;
    // for i in 0..11 {
    //     println!("{:x}", vcpu.memory[i]);
    // }
    match vcpu.read_memory_from_file("../../tests/fill.bin") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error reading binary: {}", e);
        }
    }

    loop {
        let opcode: u8 = vcpu.memory[vcpu.pc as usize];
        if opcode == cpu::OpcodeType::Hlt as u8 {
            break;
        }
        let opcode_length = vcpu.execute_instruction();

        vcpu.pc += opcode_length;
    }

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = render::App::new(768, 576, vcpu.video_memory);
    let _ = event_loop.run_app(&mut app);

    println!("{}", vcpu.registers[cpu::Register::R0 as usize]);
}
