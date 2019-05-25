#!/bin/sh

# /etc/X11/xinit/xinitrc
#
# global xinitrc file, used by all X sessions started by xinit (startx)

# Remap CapsLock to Ctrl
setxkbmap -option ctrl:nocaps

# invoke global X session script
. /etc/X11/Xsession
