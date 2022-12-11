call plug#begin('~/.config/nvim/.vim/plugged')
  " Status bar
  Plug 'vim-airline/vim-airline'

  " Color schemes
  " Plug 'morhetz/gruvbox'
  Plug 'dkasak/gruvbox'
  Plug 'NLKNguyen/papercolor-theme'

  " Fuzzy search for files
  Plug 'junegunn/fzf', { 'do': './install --bin' }
  Plug 'junegunn/fzf.vim'

  " Easy commenting
  Plug 'preservim/nerdcommenter'

  " Use git in vim
  Plug 'tpope/vim-fugitive'

  " Easy way to surround text
  Plug 'tpope/vim-surround'

  " Enables using . for some non native commands
  Plug 'tpope/vim-repeat'

  " Open and serve markdown in real time
  Plug 'iamcco/markdown-preview.nvim', { 'do': 'cd app && yarn install' }

  " Live web preview
  Plug 'turbio/bracey.vim', {'do': 'npm install --prefix server'}

  " Use code search and view
  Plug 'dyng/ctrlsf.vim'

  " Fancy start screen
  Plug 'mhinz/vim-startify'

  " Automatic session management for nvim in Lua
  Plug 'rmagatti/auto-session'

  " Code completion
  Plug 'neoclide/coc.nvim', {'branch': 'release'}

  " Syntax highlighting for elixir
  Plug 'elixir-editors/vim-elixir'

  " Syntax highlighting for haskell
  Plug 'neovimhaskell/haskell-vim'

  " Syntax highlighting for typescript
  Plug 'leafgarland/typescript-vim'

  " Display sign column for diff indication
  " Plug 'mhinz/vim-signify'

  " Display mark column
  " Plug 'chentoast/marks.nvim'
  Plug 'kshenoy/vim-signature'

  " Rainbow parentheses
  Plug 'luochen1990/rainbow'

  " Highlight word under cursor
  Plug 'dominikduda/vim_current_word'

  " Highlight hex colors
  " TODO: a bit buggy
  Plug 'norcalli/nvim-colorizer.lua'

  " Icons
  Plug 'ryanoasis/vim-devicons'

  " LaTeX support
  Plug 'lervag/vimtex'

  " Code snippets
  Plug 'honza/vim-snippets'

  " Display leader bindings
  Plug 'liuchengxu/vim-which-key'

  " Plug 'nvim-treesitter/nvim-treesitter'
  " Plug 'nvim-treesitter/nvim-treesitter-context'

  Plug 'gelguy/wilder.nvim'
  " function! UpdateRemotePlugins(...)
  "   " Needed to refresh runtime files
  "   let &rtp=&rtp
  "   UpdateRemotePlugins
  " endfunction

  " Plug 'gelguy/wilder.nvim', { 'do': function('UpdateRemotePlugins') }
call plug#end()

