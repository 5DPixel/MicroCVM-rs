use std::fs::File;
use std::io::{self, Read};

const memorySize: usize = 32 * 1024;

pub struct MicroCVMCpu {
    pub memory: Vec<u8>,
    pub registers: Vec<u16>,
    pub sp: u16,
    pub pc: u16,
    pub flags: u8,
}

pub enum OpcodeType {
    load = 0x01,
    store = 0x02,
    add = 0x03,
    sub = 0x04,
    jmp = 0x05,
    hlt = 0xFF,
    mov = 0x06,
    inc = 0x07,
    div = 0x08,
    mul = 0x09,
}

pub enum Register {
    r0 = 0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
}

pub struct Opcode {
    pub opcodeType: OpcodeType,
    pub argumentCount: u32,
    pub arg1: OpcodeArg1,
    pub arg2: OpcodeArg2,
}

pub enum OpcodeArg1 {
    Reg(Register),
    Address(u16),
}

pub enum OpcodeArg2 {
    Reg(Register),
    Immediate(i32),
    Address(u16),
}

impl MicroCVMCpu {
    pub fn empty() -> Self {
        Self {
            memory: vec![0; memorySize],
            registers: [0; 8],
            sp: 0,
            pc: 0,
            flags: 0,
        }
    }
}
