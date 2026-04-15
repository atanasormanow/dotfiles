# dotfiles-tui

My dotfiles + a TUI tool for managing dotfiles on Linux. Keeps configs in `dotfiles/` and the tool symlinks them to their destinations.

## Install & Run
Clone this repo. Delete any of my configs if you don't like them and add your own.

```bash
cargo build --release
./target/release/dotfiles-tui
```


## How It Works

The tool manages dotfiles by storing them in a central `dotfiles/` directory and creating symlinks to their target locations. Each dotfile entry consists of:

```
dotfiles/<name>/
├── <config>    # The actual file or directory
└── dest        # Target path (e.g., $HOME/.config/app/config)
```

The `dest` file contains a single line with the target path and supports environment variables like `$HOME`.

## Features

### Link Management

- **Link** - Create a symlink from the repository to the destination path
- **Unlink** - Remove the symlink at the destination (keeps the file in the repo)
- **Sync Links** - Bulk link/unlink multiple dotfiles at once. Opens a multi-select view where checked items will be linked and unchecked items will be unlinked

### Dotfile Operations

- **Add** - Import an existing config file into the repository. The file is moved to the repo and a symlink is created at the original location
- **Unmanage** - Move a dotfile from the repo back to its destination. The file will no longer be managed by the TUI but remains at its original location
- **Delete** - Permanently remove a dotfile from the repository (also removes the symlink)
- **Rename** - Rename a dotfile entry (automatically updates the symlink)
- **Edit Destination** - Change where a dotfile should be linked to
- **Edit File** - Open the config file in your `$EDITOR` (defaults to `vi`)

### Status Tracking

The TUI displays status indicators for each dotfile:

| Link Status | Meaning |
|-------------|---------|
| `[L]` Linked | Symlink is active and correct |
| `[-]` Unlinked | No symlink at destination |
| `[C]` Conflict | A regular file exists at destination |
| `[X]` Broken | Symlink points to wrong target |

Git status is also tracked (`[M]` modified, `[+]` staged).

### Other Features

- **Search** - Filter dotfiles by name
- **Path completion** - Tab completion when entering file paths
- **Auto-detection** - Automatically finds the dotfiles repository
