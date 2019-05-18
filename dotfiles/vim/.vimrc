syntax enable       "syntax processing

set tabstop=5       "spaces per tab
set softtabstop=4   "in edit mode
set expandtab       "tabs to spaces
set relativenumber
set showcmd         "commands in bottom bar
set wildmenu        "autocomplete for command menu
set showmatch       "matching braces
set incsearch       "search while typing
set hlsearch
set ignorecase
set smartcase
set list
set listchars=trail:•

nmap j gj
nmap k gk
nmap Y y$
nmap fa<space> :Files<space><return>
nmap <C-k> :move-2<return>
nmap <C-j> :move+1<return>

imap jj <esc>
imap kk <esc>
imap kj <esc>
imap jk <esc>

cmap Q q!
cmap rld so $MYVIMRC

au BufNewFile,BufRead *.hs imap ~ ->

"Auto-install vim-plug (--insecure if using proxy)
if empty(glob('~/.vim/autoload/plug.vim'))
    silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
        \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
    autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

call plug#begin('~/.vim/plugged')
Plug 'junegunn/fzf', { 'do': './install --bin' }
Plug 'junegunn/fzf.vim'
Plug 'ervandew/supertab'
Plug 'morhetz/gruvbox'
Plug 'elixir-lang/vim-elixir'
Plug 'vim-airline/vim-airline'
call plug#end()

colorscheme gruvbox
let g:gruvbox_contrast_dark='hard'
set background=dark

let g:airline_powerline_fonts = 1
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#left_alt_sep = ' | '
