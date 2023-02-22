#!/bin/bash

BLANK='#000000'
CLEAR='#000000'
DEFAULT='#AD5A00'
TEXT='#ffffff'
INPUT='#1DB954'
VERIFYING='#F0C550'
WRONG='#E00000'
FONT='DejaVuSansMono'
FONTB='DejaVuSansMono:Bold'

i3lock \
--insidever-color=$CLEAR    \
--ringver-color=$VERIFYING  \
--insidewrong-color=$CLEAR  \
--ringwrong-color=$WRONG    \
--inside-color=$BLANK       \
--ring-color=$DEFAULT       \
--line-color=$BLANK         \
--separator-color=$DEFAULT  \
--verif-color=$TEXT         \
--wrong-color=$TEXT         \
--keyhl-color=$INPUT        \
--bshl-color=$WRONG         \
--radius 120                \
--ring-width 8              \
--blur 5                    \
--clock                     \
--indicator                 \
--time-font=$FONTB          \
--time-size 46              \
--time-color=$TEXT          \
--time-str="%H:%M"          \
--time-pos="ix:(iy-20)"     \
--date-size 16              \
--date-font=$FONT           \
--date-color=$TEXT          \
--date-str="%A, %d.%m.%Y"   \
--date-pos="ix:(iy+20)"     \
--layout-color=$TEXT        \
--layout-font=$FONTB        \
--layout-size 18            \
--layout-pos="ix:(iy+70)"   \
--keylayout 1               \
