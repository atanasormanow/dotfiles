#!/bin/bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOTFILES_DIR="$SCRIPT_DIR/../dotfiles"

sym_target() {
    local dotfile_dir="$1"
    local dest_file="$dotfile_dir/dest"
    local dotfile_name
    dotfile_name=$(basename "$dotfile_dir")

    if [[ ! -f "$dest_file" ]]; then
        echo "Error: No 'dest' file in '$dotfile_name', skipping" >&2
        return 1
    fi

    local dest
    dest=$(envsubst < "$dest_file")
    local file_name
    file_name=$(basename "$dest")

    if [[ ! -f "$dotfile_dir/$file_name" ]]; then
        echo "Error: Source file '$file_name' not found in '$dotfile_name', skipping" >&2
        return 1
    fi

    local parent_dir
    parent_dir=$(dirname "$dest")
    mkdir -p "$parent_dir"

    ln -si "$dotfile_dir/$file_name" "$dest" && echo "Linked $dotfile_name"
}

for dir in "$DOTFILES_DIR"/*; do
    [[ -d "$dir" ]] || continue

    dotfile_name=$(basename "$dir")

    echo "Create a symlink for '$dotfile_name'?:"
    read -r prompt

    if [[ "$prompt" = "yes" || "$prompt" = "y" ]]; then
        sym_target "$dir" || true
    else
        echo "Skipped"
    fi
done
