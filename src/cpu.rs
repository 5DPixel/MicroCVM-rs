use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};

const FREE_MEMORY: usize = 2048 * 1024;
const VIDEO_MEMORY: usize = 1728 * 1024;
const REGISTER_COUNT: usize = 18;
pub const FLAG_ZERO: u16 = 0x0001;

pub struct MicroCVMCpu {
    pub memory: Vec<u16>,
    pub video_memory: Vec<super::types::Color>,
    pub registers: [u16; REGISTER_COUNT],
    pub pc: u16,
    pub flags: u16,
    pub framebuffer_width: usize,
    pub framebuffer_height: usize,
}

#[repr(u16)]
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
    Je = 0x0B,
    Jne = 0x0C,
    Cmp = 0x0D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Register {
    R0 = 0x1001,
    R1 = 0x1002,
    R2 = 0x1003,
    R3 = 0x1004,
    R4 = 0x1005,
    R5 = 0x1006,
    R6 = 0x1007,
    R7 = 0x1008,

    // Video argument registers
    V0 = 0x2001, // Red, BMP file path
    V1 = 0x2002, // Green
    V2 = 0x2003, // Blue
    V3 = 0x2004, // Line thickness
    V4 = 0x2005, // Starting x coordinate, general x coordinate
    V5 = 0x2006, // Starting y coordinate, general y coordinate
    V6 = 0x2007, // Ending x coordinate
    V7 = 0x2008, // Ending y coordinate

    Invalid = 0xFFFF,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FunctionCall {
    SetPixel = 0x13,
    DrawLine = 0x14,
    FillRect = 0x15,
    ClearScreen = 0x16,
    LoadBMP = 0x17,
}

#[derive(Debug)]
pub struct Opcode {
    pub opcode_type: OpcodeType,
    pub argument_count: u16,
    pub arg1: Option<OpcodeArgument>,
    pub arg2: Option<OpcodeArgument>,
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

#[derive(Debug)]
pub enum OpcodeArgument {
    Register(Register),
    Immediate(u16),
}

impl MicroCVMCpu {
    pub fn empty() -> Self {
        Self {
            memory: vec![0; FREE_MEMORY / 2],
            video_memory: vec![super::types::Color::new(0, 0, 0); VIDEO_MEMORY],
            registers: [0; REGISTER_COUNT],
            pc: 0,
            flags: 0,
            framebuffer_width: 768,
            framebuffer_height: 576,
        }
    }
    pub fn get_opcode_argument_count(opcode_type: OpcodeType) -> u8 {
        match opcode_type {
            OpcodeType::Inc => 1,
            OpcodeType::Mov => 2,
            OpcodeType::Jmp => 1,
            OpcodeType::Add => 2,
            OpcodeType::Sub => 2,
            OpcodeType::Div => 2,
            OpcodeType::Mul => 2,
            OpcodeType::Call => 1,
            OpcodeType::Je => 1,
            OpcodeType::Jne => 1,
            OpcodeType::Cmp => 2,
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
            current_instruction.arg1 = Some(
                Register::try_from(arg1)
                    .map(OpcodeArgument::Register)
                    .unwrap_or(OpcodeArgument::Immediate(arg1)),
            );
        }

        if current_instruction.argument_count >= 2 {
            let arg2 = self.memory[(self.pc + 2) as usize];
            current_instruction.arg2 = Some(
                Register::try_from(arg2)
                    .map(OpcodeArgument::Register)
                    .unwrap_or(OpcodeArgument::Immediate(arg2)),
            );
        }

        current_instruction
    }

    pub fn execute_instruction(&mut self) -> u16 {
        let opcode = self.create_opcode();

        match opcode.opcode_type {
            OpcodeType::Inc => {
                if let Some(OpcodeArgument::Register(reg)) = opcode.arg1 {
                    self.registers[Register::index(reg) as usize] += 1;
                }
            }

            OpcodeType::Mov => {
                if let (Some(OpcodeArgument::Register(dst)), Some(OpcodeArgument::Immediate(imm))) =
                    (opcode.arg2, opcode.arg1)
                {
                    self.registers[Register::index(dst) as usize] = imm;
                }
            }

            OpcodeType::Add => {
                if let (Some(OpcodeArgument::Register(dst)), Some(OpcodeArgument::Immediate(imm))) =
                    (opcode.arg2, opcode.arg1)
                {
                    self.registers[Register::index(dst) as usize] += imm;
                }
            }

            OpcodeType::Sub => {
                if let (Some(OpcodeArgument::Register(dst)), Some(OpcodeArgument::Immediate(imm))) =
                    (opcode.arg2, opcode.arg1)
                {
                    self.registers[Register::index(dst) as usize] -= imm;
                }
            }

            OpcodeType::Div => {
                if let (Some(OpcodeArgument::Register(dst)), Some(OpcodeArgument::Immediate(imm))) =
                    (opcode.arg2, opcode.arg1)
                {
                    self.registers[Register::index(dst) as usize] /= imm;
                }
            }

            OpcodeType::Mul => {
                if let (Some(OpcodeArgument::Register(dst)), Some(OpcodeArgument::Immediate(imm))) =
                    (opcode.arg2, opcode.arg1)
                {
                    self.registers[Register::index(dst) as usize] *= imm;
                }
            }

            OpcodeType::Load => {
                if let (
                    Some(OpcodeArgument::Register(dst)),
                    Some(OpcodeArgument::Immediate(addr)),
                ) = (opcode.arg2, opcode.arg1)
                {
                    self.registers[Register::index(dst) as usize] = self.memory[addr as usize];
                }
            }

            OpcodeType::Store => {
                if let (
                    Some(OpcodeArgument::Immediate(addr)),
                    Some(OpcodeArgument::Register(src)),
                ) = (opcode.arg2, opcode.arg1)
                {
                    self.memory[(addr / 2) as usize] =
                        self.registers[Register::index(src) as usize];
                }
            }

            OpcodeType::Jmp => {
                if let Some(OpcodeArgument::Immediate(target)) = opcode.arg1 {
                    self.pc = target;
                }
            }

            OpcodeType::Cmp => {
                if let (Some(OpcodeArgument::Register(dst)), Some(OpcodeArgument::Immediate(imm))) =
                    (opcode.arg2, opcode.arg1)
                {
                    let reg_value = self.registers[Register::index(dst)];
                    let is_equal = reg_value == imm;
                    self.set_flag(FLAG_ZERO, is_equal);
                }
            }

            OpcodeType::Je => {
                if let Some(OpcodeArgument::Immediate(target)) = opcode.arg1 {
                    if self.get_flag(FLAG_ZERO) == true {
                        self.pc = target;
                    }
                }
            }

            OpcodeType::Jne => {
                if let Some(OpcodeArgument::Immediate(target)) = opcode.arg1 {
                    if self.get_flag(FLAG_ZERO) == false {
                        self.pc = target;
                    }
                }
            }

            OpcodeType::Call => {
                if let Some(OpcodeArgument::Immediate(target)) = opcode.arg1 {
                    if target == FunctionCall::ClearScreen as u16 {
                        let _ = super::screen::DrawCommand::clear_screen(self);
                    }
                    if target == FunctionCall::FillRect as u16 {
                        let color = super::types::Color::new(
                            self.registers[Register::index(Register::V0) as usize] as u8,
                            self.registers[Register::index(Register::V1) as usize] as u8,
                            self.registers[Register::index(Register::V2) as usize] as u8,
                        );
                        let _ = super::screen::DrawCommand::fill_screen(self, color);
                    }
                    if target == FunctionCall::DrawLine as u16 {
                        let color = super::types::Color::new(
                            self.registers[Register::index(Register::V0) as usize] as u8,
                            self.registers[Register::index(Register::V1) as usize] as u8,
                            self.registers[Register::index(Register::V2) as usize] as u8,
                        );
                        let line_start = super::types::Point::new(
                            self.registers[Register::index(Register::V4) as usize] as isize,
                            self.registers[Register::index(Register::V5) as usize] as isize,
                        );
                        let line_end = super::types::Point::new(
                            self.registers[Register::index(Register::V6) as usize] as isize,
                            self.registers[Register::index(Register::V7) as usize] as isize,
                        );
                        let thickness =
                            self.registers[Register::index(Register::V3) as usize] as isize;
                        let _ = super::screen::DrawCommand::draw_line(
                            self, color, line_start, line_end, thickness,
                        );
                    }
                    if target == FunctionCall::LoadBMP as u16 {
                        let point = super::types::Point::new(
                            self.registers[Register::index(Register::V4) as usize] as isize,
                            self.registers[Register::index(Register::V5) as usize] as isize,
                        );
                        let bmp_file_path = "../../examples/rust.bmp";
                        let bmp_bytes = std::fs::read(bmp_file_path).unwrap();
                        let _ = super::screen::DrawCommand::draw_bmp(self, &bmp_bytes, point);
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

        self.memory.clear();
        self.memory.reserve(buffer.len());

        let mut iter = buffer.chunks(2);
        while let Some(chunk) = iter.next() {
            if chunk.len() == 2 {
                let word = u16::from_le_bytes([chunk[0], chunk[1]]);
                self.memory.push(word);
            }
        }

        let new_len = self.memory.len();
        if new_len < FREE_MEMORY {
            self.memory.extend(vec![0u16; FREE_MEMORY - new_len]);
        }

        Ok(bytes_read)
    }

    pub fn set_flag(&mut self, flag: u16, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn get_flag(&self, flag: u16) -> bool {
        self.flags & flag != 0
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
            0x0B => Ok(OpcodeType::Je),
            0x0C => Ok(OpcodeType::Jne),
            0x0D => Ok(OpcodeType::Cmp),
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
            "je" => Ok(OpcodeType::Je),
            "jne" => Ok(OpcodeType::Jne),
            "cmp" => Ok(OpcodeType::Cmp),
            invalid => return Err(InvalidOpcodeString(invalid.to_string())),
        }
    }
}

impl TryFrom<u16> for Register {
    type Error = InvalidRegister;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x1001 => Ok(Register::R0),
            0x1002 => Ok(Register::R1),
            0x1003 => Ok(Register::R2),
            0x1004 => Ok(Register::R3),
            0x1005 => Ok(Register::R4),
            0x1006 => Ok(Register::R5),
            0x1007 => Ok(Register::R6),
            0x1008 => Ok(Register::R7),
            //Video registers
            0x2001 => Ok(Register::V0), // Red
            0x2002 => Ok(Register::V1), // Green
            0x2003 => Ok(Register::V2), // Blue
            0x2004 => Ok(Register::V3), // Line thickness
            0x2005 => Ok(Register::V4), // Starting x coordinate
            0x2006 => Ok(Register::V5), // Starting y coordinate
            0x2007 => Ok(Register::V6), // Ending x coordinate
            0x2008 => Ok(Register::V7), // Ending y coordinate

            _ => Err(InvalidRegister(value)),
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
            0x17 => Ok(FunctionCall::LoadBMP),
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
            "fill_screen" => Ok(FunctionCall::FillRect),
            "clear_screen" => Ok(FunctionCall::ClearScreen),
            "load_bmp" => Ok(FunctionCall::LoadBMP),
            invalid => Err(InvalidFunctionCallString(invalid.to_string())),
        }
    }
}

impl Register {
    pub fn index(self) -> usize {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            //Video registers
            Register::V0 => 8,
            Register::V1 => 9,
            Register::V2 => 10,
            Register::V3 => 11,
            Register::V4 => 12,
            Register::V5 => 13,
            Register::V6 => 14,
            Register::V7 => 15,
            Register::Invalid => 255,
        }
    }
}
