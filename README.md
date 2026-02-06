# 🎬 GrainForge

**GrainForge** is a high-performance, procedural film grain synthesis tool built with **Rust**, **egui**, and **WGPU**. It provides a detailed interface for designing film grain textures with physically inspired parameters.

## 🚀 Versioning System

GrainForge follows [Semantic Versioning (SemVer)](https://semver.org/).

- **Current Version**: `0.1.0` (First Light MVP)
- **v0.x.x**: Initial development, breaking changes likely between minor versions.
- **v1.0.0+**: Stable API and feature set.

## ✨ Features (MVP v0.1.0)

- **Procedural Grain Engine**: GPU-accelerated grain generation using tailored noise algorithms (PCG, Simplex, Voronoi).
- **Physical Parameters**: Controls for grain amount (intensity), size, and seed variation.
- **Modern UI**: High-performance dark-themed interface built with `egui`.
- **Dual Mode Architecture**:
  - **Simple Mode**: High-level sliders for quick adjustments.
  - **Advanced Mode**: Full node graph control (Work in progress).
- **Undo/Redo System**: Full history tracking for all parameter changes.
- **Security & Standards**: Strict input validation and secure JSON serialization for stock definitions.

## 🛠 Installation & Build

### Prerequisites
- [Rust](https://rustup.rs/) (Stable 1.75+)
- A GPU with support for WebGPU/WGPU (Vulkan, Metal, or DX12)

### Running Locally
```bash
git clone https://github.com/yourname/grainforge.git
cd grainforge
cargo run --release
```

## 📂 Project Structure

- `src/app/`: Core application state and theme.
- `src/ui/`: UI components and panel layouts.
- `src/core/`: Data models, validation, and history manager.
- `src/engine/`: WGPU rendering and compute pipelines.
- `src/engine/shaders/`: WGSL grain and noise shaders.

## 📜 License

This project is licensed under the MIT OR Apache-2.0 License.

---
*Built with ❤️ by the GrainForge team.*
