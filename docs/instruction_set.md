# MicroCVM Instruction Set

MicroCVM uses a custom bytecode format with a fixed instruction encoding layout.

---

# Instruction Format

Each instruction begins with a 1-byte opcode, followed by 0–2 arguments.


- `opcode` – the operation code (1 byte)
- `arg1` – either a register index (0–7) or memory address
- `arg2` – either a register index (0–7) or immediate value

Note: If an instruction only takes 1 argument, only `arg1` is present.

---

## Registers

There are 8 general-purpose registers: `r0` through `r7`.

| Register | ID  |
|----------|-----|
| r0       | 0x00 |
| r1       | 0x01 |
| r2       | 0x02 |
| r3       | 0x03 |
| r4       | 0x04 |
| r5       | 0x05 |
| r6       | 0x06 |
| r7       | 0x07 |

---

There are also 8 video-argument registers: `v0` through `v7`.

| Register | ID  |
|----------|-----|
| v0       | 0x08 |
| v1       | 0x09 |
| v2       | 0x0A |
| v3       | 0x0B |
| v4       | 0x0C |
| v5       | 0x0D |
| v6       | 0x0E |
| v7       | 0x0F |

---

## Opcodes

| Mnemonic | Opcode (Hex) | Arguments | Description                          |
|----------|--------------|-----------|--------------------------------------|
| `hlt`    | `0xFF`       | 0         | Halts the CPU                        |
| `inc`    | `0x00`       | reg       | Increments a register                |
| `mov`    | `0x01`       | reg, imm  | Sets a register to an immediate value |
| `add`    | `0x02`       | reg, imm  | Adds an immediate to a register      |
| `sub`    | `0x03`       | reg, imm  | Subtracts an immediate from a register |
| `div`    | `0x08`       | reg, imm  | Divides an immediate with a register |
| `mul`    | `0x09`       | reg, imm  | Multiplies an immediate with a register |

---

## Examples

| Assembly        | Machine Code (Hex) | Description                     |
|-----------------|--------------------|---------------------------------|
| `inc r0`        | `0x00 0x00`            | Increment register r0           |
| `mov r1, 10`    | `0x01 0x01 0x0A`         | Move 10 into register r1        |
| `add r2, 5`     | `0x02 0x02 0x05`         | Add 5 to register r2            |
| `sub r3, 1`     | `0x03 0x03 0x01`         | Subtract 1 from register r3     |
| `hlt`           | `0xFF`               | Stop execution                  |

---

## Instruction Lengths

| Mnemonic | Bytes |
|----------|-------|
| `hlt`    | 1     |
| `inc`    | 2     |
| `mov`    | 3     |
| `add`    | 3     |
| `sub`    | 3     |

---

## Notes

- All instructions are **little-endian**.
- Only register indices 0–7 are valid. Any other value may be interpreted as an address or immediate.
