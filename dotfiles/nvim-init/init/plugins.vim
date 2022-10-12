call plug#begin('~/.config/nvim/.vim/plugged')
  " Status bar
  Plug 'vim-airline/vim-airline'

  " Color schemes
  Plug 'morhetz/gruvbox'
  Plug 'NLKNguyen/papercolor-theme'

  " Fuzzy search for files
  Plug 'junegunn/fzf', { 'do': './install --bin' }
  Plug 'junegunn/fzf.vim'

  " Easy commenting
  Plug 'preservim/nerdcommenter'

  " Easy way to surround text
  Plug 'tpope/vim-surround'

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

  " Syntax highlighting for elixir
  Plug 'elixir-editors/vim-elixir'

  " Display sign column for diff indication
  " Plug 'mhinz/vim-signify'

  " Display mark column
  " Plug 'chentoast/marks.nvim'
  Plug 'kshenoy/vim-signature'

  " Rainbow parentheses
  Plug 'luochen1990/rainbow'

  " Highlight word under cursor
  Plug 'dominikduda/vim_current_word'

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
