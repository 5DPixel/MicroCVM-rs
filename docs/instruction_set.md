# MicroCVM Instruction Set

MicroCVM uses a custom bytecode format with a fixed instruction encoding layout.

---

# 📝 Instruction Format

Each instruction begins with a 2-byte opcode, followed by 0–2 arguments.


- `opcode` – the operation code (1 byte)
- `arg1` – either a register index (0–16) or memory address
- `arg2` – either a register index (0–16) or immediate value (16-bit)

Note: If an instruction only takes 1 argument, only `arg1` is present.

---

## 🔢 Registers

There are 8 general-purpose registers: `r0` through `r7`.

| Register | ID  |
|----------|-----|
| r0       | 0x1001 |
| r1       | 0x1002 |
| r2       | 0x1003 |
| r3       | 0x1004 |
| r4       | 0x1005 |
| r5       | 0x1006 |
| r6       | 0x1007 |
| r7       | 0x1008 |

---

There are also 8 video-argument registers: `v0` through `v7`.

| Register | ID  |
|----------|-----|
| v0       | 0x2001 |
| v1       | 0x2002 |
| v2       | 0x2003 |
| v3       | 0x2004 |
| v4       | 0x2005 |
| v5       | 0x2006 |
| v6       | 0x2007 |
| v7       | 0x2008 |

---

## 🔌 Opcodes

| Mnemonic | Opcode (Hex) | Arguments | Description                          |
|----------|--------------|-----------|--------------------------------------|
| `load`    | `0x01`      | reg       | Loads an address into memory         |
| `store`    | `0x02`     | reg, imm  | Stores an address into memory        |
| `add`    | `0x03`       | reg, imm  | Adds an immediate value to a register|
| `sub`    | `0x04`       | reg, imm  | Subtracts an immediate value from a register|
| `jmp`    | `0x05`       | addr      | Jumps to an address or loop (not implemented yet)|
| `hlt`    | `0xFF`       | 0         | Terminates CPU instruction           |
| `mov`    | `0x06`       | reg, imm  | Sets a register to an immediate value|
| `inc`    | `0x07`       | reg       | Increments a register by 1           |
| `div`    | `0x08`       | reg, imm  | Divides a register by an immediate value|
| `mul`    | `0x09`       | reg, imm  | Multiplies a register by an immediate value|
| `nop`    | `0x90`       | 0         | Does nothing                         |
| `call`    | `0x0A`       | addr  | Calls an address                        |

---

## 💻 Examples

| Assembly        | Machine Code (Hex) | Description                     |
|-----------------|--------------------|---------------------------------|
| `inc r0`        | `0x00 0x10 0x01 0x00`| Increment register r0           |
| `mov 10, r1`    | `0x01 0x0A 0x10 0x02`| Move 10 into register r1        |
| `add 5, r2`     | `0x02 0x05 0x10 0x03`| Add 5 to register r2            |
| `sub 1, r3`     | `0x03 0x01 0x10 0x04`| Subtract 1 from register r3     |
| `hlt`           | `0xFF 0x00`          | Stop execution                  |

---

## 📏 Instruction Lengths

| Mnemonic | Length (bytes) |
|----------|----------------|
| load     | 2              |
| store    | 5              |
| add      | 5              |
| sub      | 5              |
| jmp      | 3              |
| hlt      | 1              |
| mov      | 5              |
| inc      | 2              |
| div      | 5              |
| mul      | 5              |
| nop      | 1              |
| call     | 3              |

---

## 📒 Notes

- All instructions are **little-endian**.
- Only register indices 0–16 are valid. Any other value may be interpreted as an address or immediate.
