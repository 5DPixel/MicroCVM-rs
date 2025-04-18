# MicroCVM

**MicroCVM** is a lightweight retro-style virtual machine written in Rust, featuring a custom 16-bit CPU, virtual memory, video RAM, and its own instruction set.

Originally written in C, this Rust version continues the minimalist spirit of the original with improved performance and modern tooling.

---

## 🔧 Features

- Custom 16-bit virtual CPU
- 2MB RAM and 1.7MB video memory
- Framebuffer-based graphics output
- Simple binary executable format
- Easy to embed, debug, and extend
- Cross-platform and fast

---

## ✍️ Writing Programs

Use the official assembler:  
👉 [**MicroCVM-rs-Assembler**](https://github.com/5DPixel/MicroCVM-rs-assembler)  
It assembles human-readable `.asm` files into executable binaries that run on the VM.

---

## 📕 Documentation

For further documentation, visit [docs](/docs)

---

## 📦 Download

You can:

- 📁 **Download precompiled binaries** from the [Releases page](https://github.com/5DPixel/MicroCVM-rs/releases)
- 📦 **Use the portable version** (no installation required)
- 🛠️ **Build from source** (see below)

---

## 🛠️ Building from Source

Requires Rust and Cargo.

```bash
git clone https://github.com/5DPixel/MicroCVM-rs
cd MicroCVM-rs
cargo build --release