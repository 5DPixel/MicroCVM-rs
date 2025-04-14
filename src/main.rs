mod cpu;
mod disk;

fn main() {
    let mut vcpu = cpu::MicroCVMCpu::empty();
    let mut vdisk = disk::MicroCVMDisk::empty();

    vcpu.memory[0] = cpu::OpcodeType::Mov as u8;
    vcpu.memory[1] = cpu::Register::R0 as u8;
    vcpu.memory[2] = 16;
    vcpu.memory[3] = cpu::OpcodeType::Hlt as u8;

    loop {
        let opcode: u8 = vcpu.memory[vcpu.pc as usize];
        if opcode == cpu::OpcodeType::Hlt as u8 {
            break;
        }
        vcpu.execute_instruction();

        vcpu.pc += 1;
    }

    println!("{}", vcpu.registers[cpu::Register::R0 as usize]);
}
