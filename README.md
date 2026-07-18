# ZED-fluxide — Official Flux IDE (Based on Zed)

A fork of the [Zed editor](https://github.com/zed-industries/zed) transformed into the official IDE for the **Flux programming language** — a compiled systems language with arbitrary-width integers, packed data layouts, bit slicing, zero-copy reinterpretation, compile-time code generation, contracts, and opt-in ownership.

## Features

| Feature | Crate | Description |
|---------|-------|-------------|
| **Data Inspector** | `flux_data_inspector` | Real-time type info panel: logical/storage width, signedness, endianness, alignment, min/max values, bit/hex masks |
| **Arbitrary-Width Visualizer** | `flux_bit_slice` | Visualize `data{5}`, `data{13}`, `data{71}` — storage layout, padding, masks, overflow behavior |
| **Bit Slice Explorer** | `flux_bit_slice` | Interactive `value[5\`18]` bit range selection with drag-to-edit |
| **Binary Transformation Viewer** | `flux_visualizer` | Live pipeline: `Packet` → `byte[]` → hex → binary |
| **`from` Reinterpretation Explorer** | `flux_struct_visualizer` | Zero-copy struct overlay: which bytes map to which fields |
| **Packed Struct Visualizer** | `flux_struct_visualizer` | Memory layout with offsets, bytes, bits, padding, alignment, nested fields |
| **Endianness Simulator** | `flux_endianness` | Toggle LE/BE with animated byte movement |
| **Binary Packet Playground** | `flux_binary_playground` | Paste hex → decode to Flux structs → bidirectional edit |
| **Compile-Time Visualizer** | `flux_comptime_viz` | Parser → AST → Semantic → Contracts → Templates → EmitFlux → LLVM → Binary |
| **Contract Inspector** | `flux_contract_inspector` | Show satisfied/failed contracts with evaluated expressions |
| **Template Geometry Viewer** | `flux_template_viz` | Template params → current types → constraint validation → generated instance |
| **Binary Explorer** | `flux_binary_explorer` | ELF sections, symbols, relocations, debug info, Flux metadata |
| **Build Dashboard** | `flux_build_dashboard` | Compile times, binary size, optimization level, cache stats |
| **Project Analyzer** | `flux_project_analyzer` | Unused types/contracts, largest structs, template usage, hotspots |
| **Native Binary Editor** | `flux_native_binary_editor` | Edit ELF/BMP/PNG/packets/firmware with Flux struct overlays |

## Architecture

```
ZED-fluxide/
├── Cargo.toml                    # Workspace with 22 Flux crates + tree-sitter-flux
├── crates/
│   ├── flux_parser/              # Tree-sitter grammar, highlights, indents, folds, locals
│   ├── flux_lsp/                 # Language Server Protocol implementation
│   ├── flux_binary_view/         # ELF/object file parsing (goblin, object)
│   ├── flux_visualizer/          # Core visualization engine
│   ├── flux_memory/              # Memory layout types, bit manipulation
│   ├── flux_debugger/            # DAP integration (gimli, addr2line)
│   ├── flux_build/               # FPM/build system integration
│   ├── flux_data_inspector/      # Feature 1: Type inspector panel
│   ├── flux_bit_slice/           # Feature 3: Bit slice explorer
│   ├── flux_struct_visualizer/   # Feature 6: Packed struct visualizer
│   ├── flux_endianness/          # Feature 7: Endianness simulator
│   ├── flux_binary_playground/   # Feature 8: Binary packet playground
│   ├── flux_comptime_viz/        # Feature 9: Compile-time visualizer
│   ├── flux_contract_inspector/  # Feature 10: Contract inspector
│   ├── flux_template_viz/        # Feature 11: Template geometry viewer
│   ├── flux_binary_explorer/     # Feature 12: Binary explorer
│   ├── flux_build_dashboard/     # Feature 13: Build dashboard
│   ├── flux_project_analyzer/    # Feature 14: Project analyzer
│   ├── flux_native_binary_editor/ # Feature 15: Native binary editor
│   ├── flux_ide/                 # Main integration: panels, language registration
│   └── tree-sitter-flux/         # Tree-sitter grammar wrapper (parser.c)
└── README.md
```

## Quick Start

### Prerequisites
- Rust 1.80+
- LLVM 21+
- Zed source code at `../zed` (path dependency)

### Build
```bash
cd ZED-fluxide
cargo build --release -p flux_ide
```

### Run
```bash
cargo run --release -p flux_ide
```

## Installation for Beginners

### Step 1: Install Rust
```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Windows
# Download from https://rustup.rs/
```

### Step 2: Install LLVM 21+
```bash
# Ubuntu/Debian
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 21

# macOS (Homebrew)
brew install llvm@21

# Windows
# Download from https://github.com/llvm/llvm-project/releases
```

### Step 3: Clone Zed and Flux IDE
```bash
# Clone Zed (required as path dependency)
git clone https://github.com/zed-industries/zed.git
cd zed

# In another terminal, clone Flux IDE
git clone https://github.com/Hemansh2633B/ZED-fluxc
cd ZED-fluxc
```

### Step 4: Build and Run
```bash
# From ZED-fluxc directory
cargo build --release -p flux_ide
cargo run --release -p flux_ide
```

## Integration with Zed

This workspace uses **path dependencies** to the Zed monorepo at `../zed`:

```toml
gpui = { path = "../zed/crates/gpui", default-features = false }
language = { path = "../zed/crates/language" }
lsp = { path = "../zed/crates/lsp" }
# ... all Zed crates
```

This means:
- Zero duplication of Zed's core crates
- Automatic updates when Zed changes
- Full compatibility with Zed's plugin system

## Flux Language Support

The `flux_parser` crate provides:
- **Tree-sitter grammar** (`tree-sitter-flux` with pre-compiled `parser.c`)
- **Syntax highlighting** (keywords, types, operators, bit slices, comptime blocks, etc.)
- **Indentation rules** for Flux constructs
- **Folding** for functions, structs, comptime blocks
- **Local variable scoping** for semantic analysis
- **Language configuration** (brackets, comments, runnables, outline, symbols)

## UI Panels (GPUI)

| Panel | Location | Purpose |
|-------|----------|---------|
| `FluxInspectorPanel` | Right sidebar | Type info on cursor hover |
| `FluxBinaryViewerPanel` | Right sidebar | ELF/binary file inspection |
| `FluxBuildPanel` | Bottom panel | Build metrics, project analysis |

All panels use Zed's **GPUI** framework for 120 FPS, GPU-accelerated rendering.

## Development

### Add a new Flux feature
1. Create crate in `crates/flux_<feature>/`
2. Add to workspace `Cargo.toml` members
3. Implement GPUI panel in `flux_ide/src/panels.rs`
4. Register in `flux_ide/src/lib.rs`

### Test Flux parsing
```bash
cargo test -p flux_parser
```

## License

GPL-3.0-or-later (same as Zed)

## Credits

- **Zed Industries** — Original editor framework
- **Flux Language Team** — Language design and compiler
- **Tree-sitter** — Incremental parsing