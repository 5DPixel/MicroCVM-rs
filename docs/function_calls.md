## 🎨 Video Registers (V0–V7)

These registers are used to pass arguments to the video system, such as drawing shapes or pixels.

| Register | Hex Code | Purpose             |
|----------|----------|---------------------|
| `v0`     | `0x2001` | Red component (0–255)       |
| `v1`     | `0x2002` | Green component (0–255)     |
| `v2`     | `0x2003` | Blue component (0–255)      |
| `v3`     | `0x2004` | Line thickness (pixels)     |
| `v4`     | `0x2005` | Start X coordinate           |
| `v5`     | `0x2006` | Start Y coordinate           |
| `v6`     | `0x2007` | End X coordinate             |
| `v7`     | `0x2008` | End Y coordinate             |

Each video register mnemonic will serve as a key to the next section.
The notation used in the next section, for example `v0 - v7` means every register, from v0 to v7

---

## 📞 Function Calls

MicroCVM supports basic function calls using the `call` instruction. A function call calls a function at a particular memory address (e.g. filling the screen, clearing it)

| Mnemonic | Opcode (Hex) | Arguments | Description                         |
|----------|--------------|-----------|-------------------------------------|
| `set_pixel`| `0x13`     | none yet  | Sets a pixel on the screen to a specified color |
| `draw_line`| `0x14`     | v0 - v7  | Calls a function at the given address |
| `fill_screen` | `0x15`  | v0 - v2  | Fills the screen to a specified RGB color |
| `clear_screen` | `0x16` | none  | Clears the screen, removing all pixel data |

---

## 💻 Examples

### ✏️ Drawing a Line – Example Assembly

This example draws a line from (10, 20) to (100, 200) in red, 3 pixels thick.

| Assembly         | Description                                 |
|------------------|---------------------------------------------|
| `mov 255, V0`    | Set red component to 255 (full red)         |
| `mov 0, V1`      | Set green component to 0                    |
| `mov 0, V2`      | Set blue component to 0                     |
| `mov 3, V3`      | Set line thickness to 3 pixels              |
| `mov 10, V4`     | Start X coordinate                          |
| `mov 20, V5`     | Start Y coordinate                          |
| `mov 100, V6`    | End X coordinate                            |
| `mov 200, V7`    | End Y coordinate                            |
| `call draw_line`    | Call line-drawing function at address 0x3000|
