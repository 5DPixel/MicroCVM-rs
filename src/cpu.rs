use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};

const FREE_MEMORY: usize = 2048 * 1024;
const VIDEO_MEMORY: usize = 1728 * 1024;
const REGISTER_COUNT: usize = 18;

pub struct MicroCVMCpu {
    pub memory: Vec<u8>,
    pub video_memory: Vec<super::types::Color>,
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
    Call = 0x0B,
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    R0 = 0x00,
    R1 = 0x01,
    R2 = 0x02,
    R3 = 0x03,
    R4 = 0x04,
    R5 = 0x05,
    R6 = 0x06,
    R7 = 0x07,
    // Video argument registers
    V0 = 0x08, //Red
    V1 = 0x09, //Green
    V2 = 0x0A, //Blue
    V3 = 0x0B, //Alpha (if applicable)
    A4 = 0x0C, //Line thickness
    V5 = 0x0D, //Starting x coordinate
    V6 = 0x0E, //Starting y coordinate
    V7 = 0x0F, //Ending x coordinate
    V8 = 0x10, //Ending y coordinate
    Invalid = 0xFF,
}

pub struct Opcode {
    pub opcode_type: OpcodeType,
    pub argument_count: u8,
    pub arg1: Option<OpcodeArg1>,
    pub arg2: Option<OpcodeArg2>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidOpcode(pub u8);

impl Display for InvalidOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Opcode: {}", self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidVideoOpcode(pub u8);

impl Display for InvalidVideoOpcode {
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

pub enum OpcodeArg1 {
    Register(Register),
    Address(u8),
}

pub enum OpcodeArg2 {
    Register(Register),
    Immediate(u8),
    Address(u8),
}

impl MicroCVMCpu {
    pub fn empty() -> Self {
        Self {
            memory: vec![0; FREE_MEMORY],
            video_memory: vec![super::types::Color::new(0, 0, 0); VIDEO_MEMORY],
            registers: vec![0; REGISTER_COUNT],
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
            let arg1 = self.memory[(self.pc + 1) as usize];
            current_instruction.arg1 = Some(if arg1 < 19 {
                OpcodeArg1::Register(Register::try_from(arg1).unwrap_or(Register::Invalid))
            } else {
                OpcodeArg1::Address(arg1)
            });
        }

        if current_instruction.argument_count >= 2 {
            let arg2 = self.memory[(self.pc + 2) as usize];
            current_instruction.arg2 = Some(if arg2 < 19 {
                OpcodeArg2::Register(Register::try_from(arg2).unwrap_or(Register::Invalid))
            } else {
                OpcodeArg2::Address(arg2)
            });
        }

        current_instruction
    }

    pub fn execute_instruction(&mut self) -> u8 {
        let opcode = self.create_opcode();

        match opcode.opcode_type {
            OpcodeType::Inc => {
                if let Some(OpcodeArg1::Register(reg)) = opcode.arg1 {
                    self.registers[reg as usize] += 1;
                }
            }

            OpcodeType::Mov => {
                if let (Some(OpcodeArg1::Register(dst)), Some(OpcodeArg2::Address(imm))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.registers[dst as usize] = imm;
                }
            }

            OpcodeType::Add => {
                if let (Some(OpcodeArg1::Register(dst)), Some(OpcodeArg2::Address(imm))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.registers[dst as usize] += imm;
                }
            }

            OpcodeType::Sub => {
                if let (Some(OpcodeArg1::Register(dst)), Some(OpcodeArg2::Address(imm))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.registers[dst as usize] -= imm;
                }
            }

            OpcodeType::Div => {
                if let (Some(OpcodeArg1::Register(dst)), Some(OpcodeArg2::Address(imm))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.registers[dst as usize] /= imm;
                }
            }

            OpcodeType::Mul => {
                if let (Some(OpcodeArg1::Register(dst)), Some(OpcodeArg2::Address(imm))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.registers[dst as usize] *= imm;
                }
            }

            OpcodeType::Load => {
                if let (Some(OpcodeArg1::Register(dst)), Some(OpcodeArg2::Address(addr))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.registers[dst as usize] = self.memory[addr as usize];
                }
            }

            OpcodeType::Store => {
                if let (Some(OpcodeArg1::Address(addr)), Some(OpcodeArg2::Register(src))) =
                    (opcode.arg1, opcode.arg2)
                {
                    self.memory[addr as usize] = self.registers[src as usize];
                }
            }

            OpcodeType::Jmp => {
                if let Some(OpcodeArg1::Address(target)) = opcode.arg1 {
                    self.pc = target;
                }
            }

            _ => {}
        }

        opcode.argument_count + 1
    }

    pub fn read_memory_from_file(&mut self, file_path: &str) -> io::Result<usize> {
        let mut file = File::open(file_path)?;

        self.memory.clear();

        let bytes_read = file.read_to_end(&mut self.memory)?;

        Ok(bytes_read)
    }
}

impl Opcode {
    pub fn empty() -> Self {
        Self {
            opcode_type: OpcodeType::Nop,
            argument_count: 0,
            arg1: None,
            arg2: None,
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
            8 => Ok(Register::V0),  // Red
            9 => Ok(Register::V1),  // Green
            10 => Ok(Register::V2), // Blue
            11 => Ok(Register::V3), // Alpha
            12 => Ok(Register::A4), // Line thickness
            13 => Ok(Register::V5), // Starting x coordinate
            14 => Ok(Register::V6), // Starting y coordinate
            15 => Ok(Register::V7), // Ending x coordinate
            16 => Ok(Register::V8), // Ending y coordinate
            invalid => return Err(InvalidRegister(invalid)),
        }
    }
}
