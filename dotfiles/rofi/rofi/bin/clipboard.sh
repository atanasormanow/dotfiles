#!/usr/bin/env bash

cliphist list | rofi -dmenu -display-columns 2 -theme ~/.config/rofi/configs/clipboard.rasi | cliphist decode | wl-copy
