#!/bin/bash

# $1 - target dotfile path
# $2 - new folder name

FILE_NAME=$(echo $1 | rev | cut -d '/' -f 1 | rev)
DEST=$(eval echo $1)

mkdir dotfiles/$2
mv -i $1 dotfiles/$2/
echo $DEST | sed "s/${USER}/\$USER/" > dotfiles/$2/dest
ln -s $(pwd)/dotfiles/$2/$FILE_NAME $DEST
