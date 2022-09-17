call plug#begin('~/.config/nvim/.vim/plugged')
  " Status bar
  Plug 'vim-airline/vim-airline'

  " Color scheme
  Plug 'morhetz/gruvbox'

  " Fuzzy search for files
  Plug 'junegunn/fzf', { 'do': './install --bin' }
  Plug 'junegunn/fzf.vim'

  " Easy commenting
  Plug 'preservim/nerdcommenter'

  " Open and serve markdown in real time
  Plug 'iamcco/markdown-preview.nvim', { 'do': 'cd app && yarn install' }

  " Use git in vim
  Plug 'tpope/vim-fugitive'

  " Use code search and view
  Plug 'dyng/ctrlsf.vim'

  " Fancy start screen
  Plug 'mhinz/vim-startify'

  " Code completion
  Plug 'neoclide/coc.nvim', {'branch': 'release'}

  " Use tab for autocompletion
  Plug 'ervandew/supertab'

  " Automatic session management
  Plug 'rmagatti/auto-session'

  " Icons
  Plug 'ryanoasis/vim-devicons'

  " LaTeX support
  Plug 'lervag/vimtex'

  " Code snippets
  Plug 'honza/vim-snippets'

  " Display leader bindings
  Plug 'liuchengxu/vim-which-key'
call plug#end()
