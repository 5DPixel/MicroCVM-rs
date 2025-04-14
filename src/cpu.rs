use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, Read};

const FREE_MEMORY: usize = 24 * 1024;
const VIDEO_MEMORY: usize = 24 * 1024;

pub struct MicroCVMCpu {
    pub memory: Vec<u8>,
    pub video_memory: Vec<u8>,
    pub registers: Vec<u8>,
    pub sp: u8,
    pub pc: u8,
    pub flags: u8,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpcodeType {
    Load = 0x01,
    Store = 0x02,
    Add = 0x03,
    Sub = 0x04,
    Jmp = 0x05,
    Hlt = 0xFF,
    Mov = 0x06,
    Inc = 0x07,
    Div = 0x08,
    Mul = 0x09,
    Nop = 0x90,
}

pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    Invalid,
}

impl Copy for Register {}

impl Clone for Register {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Opcode {
    pub opcode_type: OpcodeType,
    pub argument_count: u8,
    pub arg1: OpcodeArg1,
    pub arg2: OpcodeArg2,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidOpcode(pub u8);

impl Display for InvalidOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Opcode: {}", self.0)
    }
}

pub struct InvalidRegister(pub u8);

impl Display for InvalidRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Register: {}", self.0)
    }
}

pub union OpcodeArg1 {
    reg: Register,
    address: u8,
}

pub union OpcodeArg2 {
    reg: Register,
    immediate: u8,
    address: u8,
}

impl MicroCVMCpu {
    pub fn empty() -> Self {
        Self {
            memory: vec![0; FREE_MEMORY],
            video_memory: vec![0; VIDEO_MEMORY],
            registers: vec![0; 8],
            sp: 0,
            pc: 0,
            flags: 0,
        }
    }
    pub fn get_opcode_argument_count(opcode_type: OpcodeType) -> u8 {
        match opcode_type {
            OpcodeType::Inc => 1,
            OpcodeType::Mov => 2,
            OpcodeType::Add => 2,
            OpcodeType::Sub => 2,
            OpcodeType::Div => 2,
            OpcodeType::Mul => 2,
            _ => 0,
        }
    }

    pub fn create_opcode(&mut self) -> Opcode {
        let mut current_instruction = Opcode::empty();
        let opcode_byte: u8 = self.memory[self.pc as usize];

        current_instruction.opcode_type =
            OpcodeType::try_from(opcode_byte).unwrap_or(OpcodeType::Nop);

        current_instruction.argument_count =
            Self::get_opcode_argument_count(current_instruction.opcode_type);

        if current_instruction.argument_count >= 1 {
            let arg1: u8 = self.memory[(self.pc + 1) as usize];
            if arg1 < 8 {
                current_instruction.arg2.reg =
                    Register::try_from(arg1).unwrap_or(Register::Invalid);
            } else {
                current_instruction.arg1.address = arg1;
            }
        }

        if current_instruction.argument_count >= 2 {
            let arg2: u8 = self.memory[(self.pc + 2) as usize];
            if arg2 < 8 {
                current_instruction.arg2.reg =
                    Register::try_from(arg2).unwrap_or(Register::Invalid);
            } else {
                current_instruction.arg2.address = arg2;
            }
        }

        current_instruction
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.create_opcode();

        match opcode.opcode_type {
            OpcodeType::Inc => unsafe {
                self.registers[opcode.arg1.reg as usize] += 1;
            },
            OpcodeType::Mov => unsafe {
                self.registers[opcode.arg1.reg as usize] = opcode.arg2.immediate;
            },
            OpcodeType::Add => unsafe {
                self.registers[opcode.arg1.reg as usize] += opcode.arg2.immediate;
            },
            OpcodeType::Sub => unsafe {
                self.registers[opcode.arg1.reg as usize] -= opcode.arg2.immediate;
            },
            OpcodeType::Div => unsafe {
                self.registers[opcode.arg1.reg as usize] /= opcode.arg2.immediate;
            },
            OpcodeType::Mul => unsafe {
                self.registers[opcode.arg1.reg as usize] *= opcode.arg2.immediate;
            },
            OpcodeType::Load => {
                //To implement
            }
            OpcodeType::Store => {
                //To implement
            }
            OpcodeType::Jmp => {
                //To implement
            }

            OpcodeType::Nop => {}
            OpcodeType::Hlt => {}
        }
    }
}

impl Opcode {
    pub fn empty() -> Self {
        Self {
            opcode_type: OpcodeType::Nop,
            argument_count: 0,
            arg1: OpcodeArg1 { reg: Register::R0 },
            arg2: OpcodeArg2 { reg: Register::R0 },
        }
    }
}

impl TryFrom<u8> for OpcodeType {
    type Error = InvalidOpcode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(OpcodeType::Load),
            0x02 => Ok(OpcodeType::Store),
            0x03 => Ok(OpcodeType::Add),
            0x04 => Ok(OpcodeType::Sub),
            0x05 => Ok(OpcodeType::Jmp),
            0x06 => Ok(OpcodeType::Mov),
            0x07 => Ok(OpcodeType::Inc),
            0x08 => Ok(OpcodeType::Div),
            0x09 => Ok(OpcodeType::Mul),
            0xFF => Ok(OpcodeType::Hlt),
            0x90 => Ok(OpcodeType::Nop),
            invalid => return Err(InvalidOpcode(invalid)),
        }
    }
}

impl TryFrom<u8> for Register {
    type Error = InvalidRegister;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            invalid => return Err(InvalidRegister(invalid)),
        }
    }
}
