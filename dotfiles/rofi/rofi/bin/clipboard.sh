#!/bin/bash

theme="~/.config/rofi/configs/clipboard.rasi"

cliphist list | rofi -dmenu -display-columns 2 -theme ${dir} | cliphist decode | wl-copy
