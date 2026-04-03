# This is used by i3-sensible-terminal
export TERMINAL='alacritty'
# This is used by i3-sensible-pager
export PAGER='bat'
if [ -z "${DISPLAY}" ] && [ "${XDG_VTNR}" -eq 1 ]; then
  exec startx
fi
