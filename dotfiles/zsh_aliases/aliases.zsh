# Disable autocd
# NOTE not sure if this is the place for this
unsetopt autocd

alias ..='cd ..'
alias ...='cd ../..'
alias vim='nvim'
alias vi='nvim'
alias v='nvim'
alias nano='nvim'
alias cat='bat'
alias sl='exa'
alias ls='exa'
alias lt='exa -T -L'
alias ll='exa -lh'
alias la='exa -ah'
alias lla='exa -lah'
alias kys='poweroff'
alias reboot='reboot'
alias e='exit'
alias rsn='redshift -O 3000K'
alias rsf='redshift -x'
alias netl='nmcli device wifi list'
alias feh='feh --keep-zoom-vp'
alias nocaps='setxkbmap -option ctrl:nocaps'
alias du='du -h'
alias p8='ping 8.8.8.8'
alias zath='zathura --fork'
alias cal='cal -m3'
alias nmrestart='systemctl restart NetworkManager.service'

# Functions:
mpv-nohup () {
  nohup mpv --sub-auto=all $1 &
}

makesh() {
    echo "#!/bin/bash" > $1
    chmod u+x $1
    vim $1
}

suf() {
  mv $1 $1$2
}

mcd() {
  mkdir $1
  cd $1
}
