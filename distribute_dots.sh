#!/bin/bash

sym_target (){
    DEST=$(envsubst < $1/dest)
    FILE_NAME=$(echo $DEST | rev | cut -d '/' -f 1 | rev)
    sudo ln -si $(pwd)/$1/$FILE_NAME $DEST
}

for fold in $(ls -d dotfiles/*); do
    echo "Create a symlink for '$fold'?:"
    read prompt
    if [ "$prompt" == "yes" ] || [ "$prompt" == "y" ]; then
      sym_target $fold
    else
      echo "Skipped!"
    fi
done
