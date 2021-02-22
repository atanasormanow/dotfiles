## dotfiles
- Each file has to be in a separate folder with a file named "dest" containing its absolute path in the system.
- Run `distribute_dots.sh` to create symlinks of the dotfiles across the system. The user will be prompted for each symlink  whether to replace existing files.
- Run `add_dot.sh <file> <name>` to add a file to the set of dotfiles. A new directory in `dotfiles/` will be created, named `<name>`. `<file>` will be moved in that folder and a symlink will be created in its place.

***add_dot.sh adds only the $USER variable in your path. For more specific variables (e.g. non standard $ZSH) consider editing the path yourself***. (dotfiles/`<name>`/dest)
