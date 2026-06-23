# HIRage 

A terminal-based Rust compiler explorer built with Ratatui.

HIRage helps you inspect Rust code across multiple stages of the compilation pipeline directly from your terminal, making it easier to understand how Rust transforms source code into lower-level representations.

## Features

### Source Code Exploration

* Browse Rust source files within a project
* Navigate functions independently of files
* View extracted function source code

### Compiler Representation Views

* HIR (High-Level Intermediate Representation)
* MIR (Mid-Level Intermediate Representation)
* LLVM IR
* Assembly

### Interactive Terminal UI

* Keyboard-driven navigation
* File explorer
* Function explorer
* Scrollable viewer
* Multi-pane layout powered by Ratatui

### Performance

* Cached HIR, MIR, and LLVM outputs after first generation
* Fast switching between compiler views
* Function-level filtering for large outputs

## Installation

```bash
cargo install hirage
```

## Requirements

HIR and MIR views currently rely on Rust nightly compiler features.

```bash
rustup toolchain install nightly
rustup override set nightly
```

inside the target project.

## Usage

Navigate to a Rust project and run:

```bash
hirage
```

### Keybindings

| Key         | Action                      |
| ----------- | --------------------------- |
| ↑ / ↓       | Navigate files/functions    |
| Tab         | Switch focus                |
| Enter       | Open selected file/function |
| 1           | Source View                 |
| 2           | HIR View                    |
| 3           | MIR View                    |
| 4           | LLVM IR View                |
| 5           | Assembly View               |
| PgUp / PgDn | Scroll viewer               |
| q           | Quit                        |


## Known Limitations

* HIR and MIR views require a nightly Rust toolchain.
* Initial HIR/MIR generation may take several seconds for large projects.
* Projects with special build requirements (e.g. SQLx offline mode, custom build scripts, generated code) may require additional setup.

## Troubleshooting

### HIR/MIR view shows compilation output instead of the requested IR

In some cases, the first HIR or MIR generation may display compiler build output while the project is being compiled.

If HIR or MIR does not appear immediately:

1. Wait for compilation to finish.
2. Press `2` (HIR) or `3` (MIR) again.

HIRage caches generated outputs, so subsequent requests should be significantly faster.

### HIR/MIR generation fails

Ensure the target project is using a nightly Rust toolchain:

```bash
rustup toolchain install nightly
rustup override set nightly
```

inside the project directory.


## Motivation

HIRage was built to make Rust compiler internals more accessible from the terminal. Instead of manually running compiler commands and searching through large outputs, HIRage provides an interactive interface for exploring Rust's intermediate representations.

## Built With

* Rust
* Ratatui
* Crossterm
* Syn

## Roadmap

* [ ] Better LLVM IR filtering
* [ ] Improved MIR extraction
* [ ] Background compilation tasks
* [ ] Syntax highlighting
* [ ] Search support
* [ ] Multi-crate workspace support

## Contributing

Contributions, issues, and feature requests are welcome.

## License

MIT
