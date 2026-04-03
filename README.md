# dotfiles

A dotfiles manager for Linux systems, currently being rewritten as a Rust TUI application.

## Building and Running

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

Or run the release binary directly:

```bash
./target/release/dotfiles-tui
```

## Legacy Shell Scripts

The original shell scripts are available in `scripts/`

```bash
# Moves `<file_path>` into `dotfiles/<dotfile_name>/` and creates a symlink at the original location.
./scripts/add_dot.sh <file_path> <dotfile_name>

# Interactively creates symlinks for all managed dotfiles. You will be prompted for each one.
./scripts/distribute_dots.sh

# Creates a symlink for a specific dotfile. Accepts either a name (e.g., `alacritty`) or a path.
./scripts/use_dotfile.sh <dotfile_name>
```

## How Dotfiles Are Stored

Each dotfile lives in its own directory under `dotfiles/`:

```
dotfiles/alacritty/
├── alacritty.yml       # The config file
└── dest                # Contains: $HOME/.config/alacritty/alacritty.yml
```

The `dest` file contains the target path where the symlink should be created. Environment variables like `$HOME` are expanded at runtime.

You can manually add dotfiles by creating the directory structure yourself.

## Notes

- Scripts will create parent directories if they don't exist
- For system-level configs (e.g., `/etc/`), you may need to run scripts with `sudo`
- The `dest` file supports environment variables via `envsubst`
