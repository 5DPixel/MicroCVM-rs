use std::fmt;

#[derive(Debug)]
pub struct DrawCommandError {
    message: String,
}

impl DrawCommandError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for DrawCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawCommandError: {}", self.message)
    }
}

impl std::error::Error for DrawCommandError {}

pub struct DrawCommand();

impl DrawCommand {
    pub fn fill_screen(
        cpu: &mut super::cpu::MicroCVMCpu,
        color: super::types::Color,
    ) -> Result<(), DrawCommandError> {
        for i in 0..cpu.video_memory.len() {
            cpu.video_memory[i].r = cpu.registers[super::cpu::Register::V0 as usize];
            cpu.video_memory[i].g = cpu.registers[super::cpu::Register::V1 as usize];
            cpu.video_memory[i].b = cpu.registers[super::cpu::Register::V2 as usize];
        }

        Ok(())
        //Add error handling later
    }

    pub fn clear_screen(cpu: &mut super::cpu::MicroCVMCpu) -> Result<(), DrawCommandError> {
        cpu.video_memory.fill(super::types::Color::new(0, 0, 0));
        Ok(())
    }
}
