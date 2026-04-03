#!/bin/bash
set -uo pipefail

# $1 - target dotfile dir (e.g., alacritty or ../dotfiles/alacritty)

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <dotfile_name|dotfile_path>" >&2
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOTFILES_DIR="$SCRIPT_DIR/../dotfiles"

if [[ "$1" != *"/"* ]]; then
    TARGET_DIR="$DOTFILES_DIR/$1"
else
    TARGET_DIR="$1"
fi

if [[ ! -d "$TARGET_DIR" ]]; then
    echo "Error: Dotfile directory '$TARGET_DIR' not found" >&2
    exit 1
fi

if [[ ! -f "$TARGET_DIR/dest" ]]; then
    echo "Error: No 'dest' file in '$TARGET_DIR'" >&2
    exit 1
fi

DEST=$(envsubst < "$TARGET_DIR/dest")
FILE_NAME=$(basename "$DEST")

if [[ ! -f "$TARGET_DIR/$FILE_NAME" ]]; then
    echo "Error: Source file '$TARGET_DIR/$FILE_NAME' not found" >&2
    exit 1
fi

PARENT_DIR=$(dirname "$DEST")
mkdir -p "$PARENT_DIR"

ln -si "$TARGET_DIR/$FILE_NAME" "$DEST"

echo "Linked $(basename "$TARGET_DIR")"
