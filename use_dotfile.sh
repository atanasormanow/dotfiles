#!/bin/bash

# $1 - target dotfile dir

DEST=$(envsubst < $1/dest)
FILE_NAME=$(echo $DEST | rev | cut -d '/' -f 1 | rev)
sudo ln -si $(pwd)/$1/$FILE_NAME $DEST

