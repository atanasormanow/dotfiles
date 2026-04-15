#!/usr/bin/env bash
set -euo pipefail

# Rofi dependencies
PACKAGES=(
  rofi
  inetutils
  ttf-jetbrains-mono-nerd
)

echo "Installing Rofi dependencies..."

for pkg in "${PACKAGES[@]}"; do
  if ! pacman -Qi "$pkg" &>/dev/null; then
    echo "Installing $pkg..."
    sudo pacman -S --noconfirm "$pkg" || {
      echo "ERROR: Failed to install $pkg" >&2
      exit 1
    }
  else
    echo "Already installed: $pkg"
  fi
done

echo "All dependencies installed successfully"
