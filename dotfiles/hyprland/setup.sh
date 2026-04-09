#!/usr/bin/env bash
set -euo pipefail

# Core Hyprland packages
PACKAGES=(
    hyprland
    hyprpaper
    hyprlock
    xdg-desktop-portal-hyprland
    kitty
    rofi
    grim
    slurp
    wl-clipboard
    libnotify
    pulseaudio
    playerctl
    brightnessctl
)

echo "Installing Hyprland dependencies..."

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
