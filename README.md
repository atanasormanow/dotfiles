# dotfiles

A TUI tool for managing dotfiles on Linux. Tracks configs via git and symlinks them to their destinations.

## Usage

```bash
cargo build --release
./target/release/dotfiles-tui
```

## Keybindings

| Key | Action |
|-----|--------|
| `j/k` | Navigate |
| `l` | Link dotfile |
| `u` | Unlink dotfile |
| `a` | Add new dotfile |
| `d` | Delete dotfile |
| E | Edit dotfile |
| `e` | Edit destination |
| `r` | Refresh |
| `/` | Search |
| `?` | Help |
| `q` | Quit |

## Storage Format

```
dotfiles/<name>/
├── <config>    # The actual file or directory
└── dest        # Target path (e.g., $HOME/.config/app/config)
```

The `dest` file supports `$HOME` and other environment variables.
