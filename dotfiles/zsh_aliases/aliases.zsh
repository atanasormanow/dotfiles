alias vi='vim'
alias v='vim'
alias nano='vim'
alias ll='ls -lh'
alias lah='ls -lah'
alias la='ls -ah'
alias sl='ls'
alias kys='sudo shutdown now'
alias reboot='sudo reboot'
alias zshal='vim ~/.oh-my-zsh/custom/aliases.zsh'
alias pwtune='sudo powertop --auto-tune'
alias vimrc='vim ~/.vimrc'
alias e='exit'
alias mdir='mkdir -p'
alias rsn='redshift -O 2700K'
alias rsf='redshift -x'
alias os='ssh s81669@62.44.100.23'
alias netl='nmcli device wifi list'
alias feh='feh --keep-zoom-vp'
alias autore='sudo apt autoclean && sudo apt autoremove'
alias upgrade='sudo apt update && sudo apt upgrade'
alias feh='feh --edit'
alias su='su --preserve-environment'
alias nocaps='sudo setxkbmap -option ctrl:nocaps'
alias du='du -h'
alias p8='ping 8.8.8.8'
alias sx='startx'
alias ..='cd ..'
alias ...='cd ../..'

mpv-nohup () {
  nohup mpv --sub-auto=all $1 &
}

makesh() {
    echo "#!/bin/bash" > $1
    chmod u+x $1
    vim $1
}

mvv () {
  mv */$1 .
}

unsetopt autocd
