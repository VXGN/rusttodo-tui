# rusttodo-tui

A terminal-based todo list application built with Rust, featuring an intuitive Text User Interface (TUI).
nd minimalist design

## Prerequisites

- Rust 1.70 or higher
- Cargo 

## Installation

### From source

```bash
git clone https://github.com/yourusername/rusttodo-tui.git
cd rusttodo-tui
cargo build --release
```

### Using cargo install

```bash
cargo install --path .
```

## Usage

Run the application:

```bash
cargo run
```

Or if installed:

```bash
rusttodo-tui
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `a` | Add new task |
| `d` | Delete selected task |
| `Space` | Toggle task completion |
| `e` | Edit selected task |
| `↑/↓` or `j/k` | Navigate tasks |
| `q` | Quit application |

## Dependencies

- `ratatui` - TUI framework
- `crossterm` - Terminal manipulation
- `serde` - Serialization/deserialization
- `tokio` - Async runtime (if applicable)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui)
- Inspired by terminal productivity tools