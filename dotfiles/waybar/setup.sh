#!/usr/bin/env bash
set -euo pipefail

# Waybar dependencies
PACKAGES=(
  waybar
  fzf
  bluez-utils
  brightnessctl
  networkmanager
  otf-commit-mono-nerd
  bluetui
)

echo "Installing Waybar dependencies..."

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

# Restart waybar
echo "Restarting Waybar..."

if ! (systemctl --user is-enabled waybar.service &&
  systemctl --user restart waybar.service) &>/dev/null; then
  pkill waybar 2>/dev/null || true
  waybar &>/dev/null &
  disown
fi

echo "All dependencies installed successfully"
