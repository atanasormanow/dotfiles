## dotfiles
- Each file has to be in a separate folder with a file named "dest" containing
its absolute path in the system.
- Run distribute_dots.sh to create symlinks of the dotfiles across the system.
The user will be prompted for each symlink  whether to remove the destination
file.
- Run add_dot.sh `<file>` `<name>` to add a file to the set of dotfiles. A new
folder in dotfiles/ will be created with the name `<name>`. The file `<file>` will
be moved in that folder, a symlink to it will be placed in its old directory.

***add_dot.sh adds only the $USER variable in your path. For more specific variables (e.g. non standard $ZSH) consider
editing the path yourself***. (dotfiles/`<name>`/dest)
