#!/bin/bash

sym_target (){
    DEST=$(envsubst < $1/dest)
    FILE_NAME=$(echo $DEST | rev | cut -d '/' -f 1 | rev)
    ln -si $(pwd)/$1/$FILE_NAME $DEST
}

for fold in $(ls -d dotfiles/*); do
    sym_target $fold
done