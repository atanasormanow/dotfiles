# dotfiles

A dotfiles manager for Linux systems, currently being rewritten as a Rust TUI application.

## Project Structure

```
dotfiles/
├── dotfiles/           # Managed dotfile configurations
│   └── <name>/
│       ├── <config>    # The actual config file
│       └── dest        # Target path (supports $HOME, etc.)
├── scripts/            # Legacy shell scripts
├── src/                # Rust TUI source code
├── Cargo.toml          # Rust project manifest
└── Cargo.lock          # Dependency lockfile
```

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

The original shell scripts are available in `scripts/` for reference.

### Add a dotfile

```bash
./scripts/add_dot.sh <file_path> <dotfile_name>
```

Moves `<file_path>` into `dotfiles/<dotfile_name>/` and creates a symlink at the original location.

### Distribute all dotfiles

```bash
./scripts/distribute_dots.sh
```

Interactively creates symlinks for all managed dotfiles. You will be prompted for each one.

### Use a single dotfile

```bash
./scripts/use_dotfile.sh <dotfile_name>
```

Creates a symlink for a specific dotfile. Accepts either a name (e.g., `alacritty`) or a path.

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
