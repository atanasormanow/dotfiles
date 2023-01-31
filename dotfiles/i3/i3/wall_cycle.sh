#!/usr/bin/env bash

# TODO just use a cronjob 4head

#DIRECTORY: $1
#TIMEOUT(S): $1

while true; do
  feh --randomize --bg-scale $1/*
  sleep $2
done
