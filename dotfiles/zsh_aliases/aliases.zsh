# Disable autocd
# NOTE not sure if this is the place for this
unsetopt autocd

alias vi='vim'
alias v='vim'
alias nano='vim'
alias cat='bat'
alias sl='exa'
alias ls='exa'
alias lt='exa -T -L'
alias ll='exa -lh'
alias la='exa -ah'
alias lla='exa -lah'
alias kys='sudo shutdown now'
alias reboot='sudo reboot'
alias zshal='vim ~/.oh-my-zsh/custom/aliases.zsh'
alias vimrc='vim ~/.vimrc'
alias e='exit'
alias rsn='redshift -O 3000K'
alias rsf='redshift -x'
alias netl='nmcli device wifi list'
alias feh='feh --keep-zoom-vp'
alias autore='sudo apt autoclean && sudo apt autoremove'
alias upgrade='sudo apt update && sudo apt upgrade'
alias su='su --preserve-environment'
alias nocaps='sudo setxkbmap -option ctrl:nocaps'
alias du='du -h'
alias p8='ping 8.8.8.8'
alias sx='startx'
alias ..='cd ..'
alias ...='cd ../..'
alias apti='sudo apt install'
alias aptp='sudo apt purge'
alias ssp='sudo systemctl suspend'

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

autopaste() {
  /home/nakk/Workspace/playground/Bash/autopaste.sh $1 $2 $3
}

##############
# Work stuff #
##############
alias san='cd /home/nakk/Workspace/santiment/sanbase2'
alias phxi='iex --erl "-kernel shell_history enabled" -S mix phx.server'
alias wgup='sudo wg-quick up wg-client-stage'
alias wgdn='sudo wg-quick down wg-client-stage'
alias kgps='kubectl get pods | rg sanbase'
alias rldb='MIX_ENV=test mix do ecto.drop, ecto.create, ecto.load'

mcd() {
  mkdir $1
  cd $1
}

klftn(){
 kubectl logs -f --tail=5000 $1 | gsed 's/\\n/\n/g'
}

#######
# TMP #
#######
alias passc='xclip /home/nakk/Workspace/hcrem/tmp/pass'
