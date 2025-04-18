use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};

const FREE_MEMORY: usize = 2048 * 1024;
const VIDEO_MEMORY: usize = 1728 * 1024;
const REGISTER_COUNT: usize = 18;

pub struct MicroCVMCpu {
    pub memory: Vec<u16>,
    pub video_memory: Vec<super::types::Color>,
    pub registers: [u16; REGISTER_COUNT],
    pub pc: u16,
    pub framebuffer_width: usize,
    pub framebuffer_height: usize,
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
    Call = 0x0A,
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
    V3 = 0x0B, //Line thickness
    V4 = 0x0C, //Starting x coordinate
    V5 = 0x0D, //Starting y coordinate
    V6 = 0x0E, //Ending x coordinate
    V7 = 0x0F, //Ending y coordinate
    Invalid = 0xFF,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FunctionCall {
    SetPixel = 0x13,
    DrawLine = 0x14,
    FillRect = 0x15,
    ClearScreen = 0x16,
}

pub struct Opcode {
    pub opcode_type: OpcodeType,
    pub argument_count: u16,
    pub arg1: Option<OpcodeArg1>,
    pub arg2: Option<OpcodeArg2>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidOpcode(pub u16);

impl Display for InvalidOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Opcode: {}", self.0)
    }
}

pub struct InvalidOpcodeString(pub String);

impl Display for InvalidOpcodeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Opcode: {}", self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidFunctionCall(pub u16);

impl Display for InvalidFunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Function Call: {}", self.0)
    }
}

pub struct InvalidRegister(pub u16);

impl Display for InvalidRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Register: {}", self.0)
    }
}

pub struct InvalidRegisterString(pub String);

impl Display for InvalidRegisterString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Register: {}", self.0)
    }
}

pub struct InvalidFunctionCallString(pub String);

impl Display for InvalidFunctionCallString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Function Call: {}", self.0)
    }
}

pub enum OpcodeArg1 {
    Register(Register),
    Address(u16),
}

pub enum OpcodeArg2 {
    Register(Register),
    Immediate(u16),
    Address(u16),
}

impl MicroCVMCpu {
    pub fn empty() -> Self {
        Self {
            memory: vec![0; FREE_MEMORY / 2],
            video_memory: vec![super::types::Color::new(0, 0, 0); VIDEO_MEMORY],
            registers: [0; REGISTER_COUNT],
            pc: 0,
            framebuffer_width: 768,
            framebuffer_height: 576,
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
            OpcodeType::Call => 1,
            _ => 0,
        }
    }

    pub fn create_opcode(&mut self) -> Opcode {
        let mut current_instruction = Opcode::empty();

        let opcode_word = self.memory[self.pc as usize];
        let opcode_byte = (opcode_word & 0xFF) as u8;
        current_instruction.opcode_type =
            OpcodeType::try_from(opcode_byte as u16).unwrap_or(OpcodeType::Nop);

        current_instruction.argument_count =
            Self::get_opcode_argument_count(current_instruction.opcode_type) as u16;

        if current_instruction.argument_count >= 1 {
            let arg1 = self.memory[(self.pc + 1) as usize];
            current_instruction.arg1 = Some(if arg1 < 16 {
                OpcodeArg1::Register(Register::try_from(arg1).unwrap_or(Register::Invalid))
            } else {
                OpcodeArg1::Address(arg1)
            });
        }

        if current_instruction.argument_count >= 2 {
            let arg2 = self.memory[(self.pc + 2) as usize];
            current_instruction.arg2 = Some(if arg2 < 16 {
                OpcodeArg2::Register(Register::try_from(arg2).unwrap_or(Register::Invalid))
            } else {
                OpcodeArg2::Address(arg2)
            });
        }

        current_instruction
    }

    pub fn execute_instruction(&mut self) -> u16 {
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
                    self.memory[(addr / 2) as usize] = self.registers[src as usize];
                }
            }

            OpcodeType::Jmp => {
                if let Some(OpcodeArg1::Address(target)) = opcode.arg1 {
                    self.pc = target;
                }
            }

            OpcodeType::Call => {
                if let Some(OpcodeArg1::Address(target)) = opcode.arg1 {
                    if target == FunctionCall::ClearScreen as u16 {
                        let _ = super::screen::DrawCommand::clear_screen(self);
                    }
                    if target == FunctionCall::FillRect as u16 {
                        let color = super::types::Color::new(
                            self.registers[Register::V0 as usize] as u8,
                            self.registers[Register::V1 as usize] as u8,
                            self.registers[Register::V2 as usize] as u8,
                        );
                        let _ = super::screen::DrawCommand::fill_screen(self, color);
                    }
                    if target == FunctionCall::DrawLine as u16 {
                        let color = super::types::Color::new(
                            self.registers[Register::V0 as usize] as u8,
                            self.registers[Register::V1 as usize] as u8,
                            self.registers[Register::V2 as usize] as u8,
                        );
                        let line_start = super::types::Point::new(
                            self.registers[Register::V4 as usize] as isize,
                            self.registers[Register::V5 as usize] as isize,
                        );
                        let line_end = super::types::Point::new(
                            self.registers[Register::V6 as usize] as isize,
                            self.registers[Register::V7 as usize] as isize,
                        );
                        let _ = super::screen::DrawCommand::draw_line(
                            self, color, line_start, line_end,
                        );
                    }
                }
            }

            _ => {}
        }

        opcode.argument_count + 1
    }

    pub fn read_memory_from_file(&mut self, file_path: &str) -> io::Result<usize> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        let bytes_read = file.read_to_end(&mut buffer)?;

        if buffer.len() % 2 != 0 {
            buffer.push(0x00);
        }

        self.memory.clear();
        self.memory.reserve(buffer.len() / 2);

        for chunk in buffer.chunks_exact(2) {
            let word = u16::from_le_bytes([chunk[0], chunk[1]]);
            self.memory.push(word);
        }

        let new_len = self.memory.len();
        if new_len < FREE_MEMORY {
            self.memory.extend(vec![0u16; FREE_MEMORY - new_len]);
        }

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

impl TryFrom<u16> for OpcodeType {
    type Error = InvalidOpcode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
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
            0x0A => Ok(OpcodeType::Call),
            invalid => return Err(InvalidOpcode(invalid)),
        }
    }
}

impl TryFrom<&str> for OpcodeType {
    type Error = InvalidOpcodeString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "load" => Ok(OpcodeType::Load),
            "store" => Ok(OpcodeType::Store),
            "add" => Ok(OpcodeType::Add),
            "sub" => Ok(OpcodeType::Sub),
            "jmp" => Ok(OpcodeType::Jmp),
            "hlt" => Ok(OpcodeType::Hlt),
            "mov" => Ok(OpcodeType::Mov),
            "inc" => Ok(OpcodeType::Inc),
            "div" => Ok(OpcodeType::Div),
            "mul" => Ok(OpcodeType::Mul),
            "nop" => Ok(OpcodeType::Nop),
            "call" => Ok(OpcodeType::Call),
            invalid => return Err(InvalidOpcodeString(invalid.to_string())),
        }
    }
}

impl TryFrom<u16> for Register {
    type Error = InvalidRegister;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
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
            11 => Ok(Register::V3), // Line thickness
            12 => Ok(Register::V4), // Starting x coordinate
            13 => Ok(Register::V5), // Starting y coordinate
            14 => Ok(Register::V6), // Ending x coordinate
            15 => Ok(Register::V7),
            invalid => return Err(InvalidRegister(invalid)),
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = InvalidRegisterString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "r0" => Ok(Register::R0),
            "r1" => Ok(Register::R1),
            "r2" => Ok(Register::R2),
            "r3" => Ok(Register::R3),
            "r4" => Ok(Register::R4),
            "r5" => Ok(Register::R5),
            "r6" => Ok(Register::R6),
            "r7" => Ok(Register::R7),
            //Video opcodes
            "v0" => Ok(Register::V0),
            "v1" => Ok(Register::V1),
            "v2" => Ok(Register::V2),
            "v3" => Ok(Register::V3),
            "v4" => Ok(Register::V4),
            "v5" => Ok(Register::V5),
            "v6" => Ok(Register::V6),
            "v7" => Ok(Register::V7),
            invalid => Err(InvalidRegisterString(invalid.to_string())),
        }
    }
}

impl TryFrom<u16> for FunctionCall {
    type Error = InvalidFunctionCall;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x13 => Ok(FunctionCall::SetPixel),
            0x14 => Ok(FunctionCall::DrawLine),
            0x15 => Ok(FunctionCall::FillRect),
            0x16 => Ok(FunctionCall::ClearScreen),
            invalid => return Err(InvalidFunctionCall(invalid)),
        }
    }
}

impl TryFrom<&str> for FunctionCall {
    type Error = InvalidFunctionCallString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "set_pixel" => Ok(FunctionCall::SetPixel),
            "draw_line" => Ok(FunctionCall::DrawLine),
            "fill_rect" => Ok(FunctionCall::FillRect),
            "clear_screen" => Ok(FunctionCall::ClearScreen),
            invalid => Err(InvalidFunctionCallString(invalid.to_string())),
        }
    }
}
