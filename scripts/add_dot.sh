#!/bin/bash
set -uo pipefail

# $1 - target dotfile path
# $2 - new folder name

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <file_path> <dotfile_name>" >&2
    exit 1
fi

SOURCE_FILE="$1"
DOTFILE_NAME="$2"

if [[ ! -f "$SOURCE_FILE" ]]; then
    echo "Error: File '$SOURCE_FILE' not found" >&2
    exit 1
fi

FILE_NAME=$(basename "$SOURCE_FILE")
DEST="${SOURCE_FILE/#\~/$HOME}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOTFILES_DIR="$SCRIPT_DIR/../dotfiles"
TARGET_DIR="$DOTFILES_DIR/$DOTFILE_NAME"

if [[ -d "$TARGET_DIR" ]]; then
    echo "Error: Dotfile '$DOTFILE_NAME' already exists" >&2
    exit 1
fi

mkdir -p "$TARGET_DIR"
mv -i "$SOURCE_FILE" "$TARGET_DIR/"
echo "${DEST//$HOME/\$HOME}" > "$TARGET_DIR/dest"
ln -si "$TARGET_DIR/$FILE_NAME" "$DEST"

echo "Added dotfile '$DOTFILE_NAME'"
