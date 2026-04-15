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

# AUR dependencies
AUR_PACKAGES=(
  "gazelle-tui|https://aur.archlinux.org/gazelle-tui.git"
)

echo "Installing AUR dependencies..."

for entry in "${AUR_PACKAGES[@]}"; do
  pkg="${entry%%|*}"
  url="${entry##*|}"
  
  if ! pacman -Qi "$pkg" &>/dev/null; then
    echo "Installing $pkg from AUR..."
    mkdir -p ~/AUR
    if [[ -d ~/AUR/"$pkg" ]]; then
      git -C ~/AUR/"$pkg" pull
    else
      git clone "$url" ~/AUR/"$pkg"
    fi
    (cd ~/AUR/"$pkg" && makepkg -si --noconfirm) || {
      echo "ERROR: Failed to install $pkg from AUR" >&2
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
