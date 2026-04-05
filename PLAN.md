# Dotfiles TUI - Specification & Plan

## Overview

A terminal-based user interface (TUI) for managing dotfiles on Linux systems. The tool uses a git repository as storage, tracking changes and providing symlink management between the repository and system locations.

## Project Structure

```
dotfiles/
├── dotfiles/           # Managed dotfile storage
│   └── <name>/         # Each dotfile in its own directory
│       ├── <file>      # The actual config file(s)
│       └── dest        # Target path (supports $HOME, etc.)
├── scripts/            # Legacy shell scripts (reference implementation)
├── src/                # Rust TUI source code
├── Cargo.toml
└── PLAN.md             # This specification
```

---

## Core Features

### 1. Dotfile Listing (Main View)

The primary view displays all managed dotfiles in a scrollable list.

**Columns:**
| Column | Description |
|--------|-------------|
| Name | Directory name in `dotfiles/` |
| Destination | Resolved target path from `dest` file |
| Status | Symlink state indicator |
| Changes | Git status indicator |

**Status Indicators:**
- `[L]` Linked - symlink exists and points to repo
- `[X]` Broken - symlink exists but target missing
- `[C]` Conflict - file exists at destination but is not a symlink
- `[-]` Unlinked - no file/symlink at destination
- `[?]` Unknown - `dest` file missing or invalid

**Git Change Indicators:**
- `[M]` Modified - uncommitted changes
- `[+]` Staged - changes staged for commit
- `[ ]` Clean - no changes

### 2. Add Dotfile

Import an existing config file into the repository.

**Process:**
1. User provides: source file path, dotfile name
2. Validate source file exists
3. Validate dotfile name doesn't already exist
4. Create `dotfiles/<name>/` directory
5. Move source file to the new directory
6. Create `dest` file with original path (using `$HOME` where applicable)
7. Create symlink at original location pointing to repo

**Equivalent to:** `scripts/add_dot.sh`

### 3. Link Single Dotfile

Create a symlink for a specific dotfile.

**Process:**
1. User selects dotfile from list
2. Read and expand `dest` file
3. Create parent directories if needed
4. Create symlink (with confirmation if file exists)

**Equivalent to:** `scripts/use_dotfile.sh`

### 4. Distribute All Dotfiles

Batch operation to link multiple/all dotfiles.

**Process:**
1. Show checklist of all unlinked dotfiles
2. User selects which to link
3. Process each selected dotfile
4. Report results (success/failure for each)

**Equivalent to:** `scripts/distribute_dots.sh`

### 5. Edit Destination

Modify the target path for a dotfile.

**Process:**
1. User selects dotfile
2. Show current destination
3. User edits the path
4. Validate new path format
5. Update `dest` file
6. Optionally re-link to new destination

### 6. Unlink Dotfile

Remove a symlink without deleting the dotfile from repo.

**Process:**
1. User selects linked dotfile
2. Confirm action
3. Remove symlink at destination
4. Update status display

### 7. Remove Dotfile

Completely remove a dotfile from the repository.

**Process:**
1. User selects dotfile
2. Show warning about permanent deletion
3. Optionally restore file to original destination
4. Remove directory from `dotfiles/`

### 8. Git Integration

Track and manage changes through git.

**Features:**
- View uncommitted changes per dotfile
- Quick commit with message
- View git log/history
- Pull/push operations
- Diff view for modified files

---

## TUI Layout

```
┌─────────────────────────── Dotfiles TUI ───────────────────────────┐
│ [q]uit  [a]dd  [l]ink  [d]istribute  [e]dit  [u]nlink  [g]it  [?]  │
├────────────────────────────────────────────────────────────────────┤
│  Name            │ Destination                    │ Status │ Git   │
│──────────────────┼────────────────────────────────┼────────┼───────│
│> alacritty       │ ~/.config/alacritty/alacrit... │  [L]   │  [ ]  │
│  i3              │ ~/.config/i3/config            │  [L]   │  [M]  │
│  vim             │ ~/.vimrc                       │  [-]   │  [ ]  │
│  zshrc           │ ~/.zshrc                       │  [C]   │  [ ]  │
│  xorg.conf.d     │ /etc/X11/xorg.conf.d           │  [L]   │  [ ]  │
│                  │                                │        │       │
│                  │                                │        │       │
├────────────────────────────────────────────────────────────────────┤
│ alacritty: Linked to ~/.config/alacritty/alacritty.yml             │
└────────────────────────────────────────────────────────────────────┘
```

### Layout Components

1. **Title Bar** - Application name
2. **Action Bar** - Keyboard shortcuts for available actions
3. **List Header** - Column titles
4. **Dotfile List** - Scrollable, selectable list of dotfiles
5. **Status Bar** - Details about selected item, messages, errors

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up |
| `g` / `Home` | Go to first item |
| `G` / `End` | Go to last item |
| `Enter` | Open action menu for selected item |
| `/` | Search/filter dotfiles |
| `Esc` | Cancel/back |
| `q` | Quit application |

### Action Shortcuts

| Key | Action |
|-----|--------|
| `a` | Add new dotfile |
| `l` | Link selected dotfile |
| `L` | Link all (distribute) |
| `u` | Unlink selected dotfile |
| `e` | Edit destination |
| `d` | Delete dotfile (with confirmation) |
| `r` | Refresh list |
| `g` | Git menu |
| `?` | Help screen |

---

## Modal Dialogs

### Confirmation Dialog
```
┌─────────── Confirm ───────────┐
│                               │
│  Unlink 'alacritty'?          │
│                               │
│  This will remove the         │
│  symlink at:                  │
│  ~/.config/alacritty/...      │
│                               │
│     [Y]es       [N]o          │
└───────────────────────────────┘
```

### Input Dialog
```
┌────────── Add Dotfile ──────────┐
│                                 │
│  Source path:                   │
│  > ~/.config/nvim/init.lua      │
│                                 │
│  Dotfile name:                  │
│  > nvim                         │
│                                 │
│    [Enter] Confirm  [Esc] Cancel│
└─────────────────────────────────┘
```

### Multi-Select Dialog (Distribute)
```
┌────────── Distribute ──────────┐
│  Select dotfiles to link:      │
│                                │
│  [x] alacritty                 │
│  [ ] i3                        │
│  [x] vim                       │
│  [ ] zshrc (conflict)          │
│                                │
│  [Space] Toggle  [Enter] Apply │
└────────────────────────────────┘
```

---

## Security Considerations

### 1. Elevated Permissions

Some dotfiles require root access (e.g., `/etc/X11/xorg.conf.d`).

**Approach:**
- Detect when destination requires elevated permissions
- Display indicator in the list (e.g., `[S]` for sudo required)
- Prompt user before operations requiring sudo
- Use `pkexec` or similar for GUI-friendly privilege escalation
- Never store credentials

### 2. Symlink Safety

**Protections:**
- Never overwrite existing files without explicit confirmation
- Show clear diff/comparison when conflicts exist
- Backup option before destructive operations
- Validate symlink targets before creation

### 3. Path Handling

**Validations:**
- Sanitize all user input paths
- Prevent path traversal attacks (no `..` escaping repo)
- Validate environment variable expansion
- Handle special characters in filenames

### 4. Git Operations

**Safety Measures:**
- Never auto-push without user confirmation
- Show diff before committing
- Prevent commits of sensitive data (optional .gitignore suggestions)
- Handle merge conflicts gracefully

---

## Functionality Cornerstones

### 1. Repository as Single Source of Truth

- All dotfiles are stored in `dotfiles/` directory
- The `dest` file defines where each dotfile belongs
- Git tracks all changes to dotfiles
- System locations contain only symlinks

### 2. Non-Destructive by Default

- Moving files to repo creates symlinks automatically
- Unlinking removes symlink, not the repo copy
- Delete operations require explicit confirmation
- Conflicts shown clearly before any action

### 3. Environment Variable Support

- `dest` files support `$HOME` and other env vars
- Variables expanded at runtime using current environment
- Store paths with variables for portability

### 4. Graceful Degradation

- Handle missing `dest` files gracefully
- Continue operation if single dotfile fails
- Clear error messages for troubleshooting
- Legacy scripts remain functional

---

## Implementation Phases

### Phase 1: Core Infrastructure
- [ ] App state management
- [ ] Dotfile discovery and parsing
- [ ] Symlink status detection
- [ ] Basic list rendering

### Phase 2: Read-Only Features
- [ ] Main list view with all columns
- [ ] Navigation and selection
- [ ] Detailed view for selected dotfile
- [ ] Search/filter functionality

### Phase 3: Write Operations
- [ ] Link single dotfile
- [ ] Unlink dotfile
- [ ] Add new dotfile
- [ ] Distribute (batch link)

### Phase 4: Edit & Management
- [ ] Edit destination
- [ ] Remove dotfile
- [ ] Conflict resolution UI

### Phase 5: Git Integration
- [ ] Git status per dotfile
- [ ] Commit interface
- [ ] Diff viewer
- [ ] Log viewer

### Phase 6: Polish
- [ ] Help system
- [ ] Error handling improvements
- [ ] Configuration file (optional)
- [ ] Themes/colors

---

## Future Feature Ideas

- **Profiles**: Group dotfiles by machine/context (work, home, server)
- **Templates**: Support for templated configs with variable substitution
- **Hooks**: Pre/post link scripts per dotfile
- **Backup**: Automatic backup before overwriting conflicts
- **Import**: Import dotfiles from other managers (stow, chezmoi)
- **Export**: Generate shell script to reproduce setup
- **Remote sync**: Beyond git (rsync, cloud storage)
- **Dry-run mode**: Preview all operations before execution
- **Undo**: Revert recent operations

---

## Technical Notes

### Dependencies (Current)
- `ratatui` - TUI framework
- `crossterm` - Terminal backend

### Suggested Additions
- `git2` - Git operations (libgit2 bindings)
- `dirs` - Standard directory paths
- `shellexpand` - Environment variable expansion
- `walkdir` - Directory traversal
- `anyhow` / `thiserror` - Error handling

### File Operations
- Use Rust's `std::fs` with proper error handling
- Atomic operations where possible
- Handle cross-filesystem moves (copy + delete)

### Testing Strategy
- Unit tests for path manipulation, parsing
- Integration tests with temp directories
- Manual testing for TUI interactions
