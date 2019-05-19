" Auto-install vim-plug (--insecure if using proxy)
if empty(glob('~/.vim/autoload/plug.vim'))
    silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
        \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
    autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Plugins
call plug#begin('~/.vim/plugged')
Plug 'junegunn/fzf', { 'do': './install --bin' }
Plug 'junegunn/fzf.vim'
Plug 'ervandew/supertab'
Plug 'morhetz/gruvbox'
Plug 'elixir-lang/vim-elixir'
Plug 'vim-airline/vim-airline'
call plug#end()

" Plugins configuration
colorscheme gruvbox
let g:gruvbox_contrast_dark='hard'

let g:airline_powerline_fonts = 1
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#left_alt_sep = ' | '

" Vim options
syntax enable           " syntax processing
set t_Co=256            " more colors
set background=dark     " set background theme to dark
set mouse=a             " enable mouse
set relativenumber      " line numbers - relative to the cursor
set number              " current line number for relative numbers
set scrolloff=4         " show first/last lines when scrolling
set tabstop=4           " spaces per tab
set softtabstop=4       " in edit mode
set expandtab           " tabs to spaces
set showcmd             " commands in bottom bar
set wildmenu            " autocomplete for command menu
set showmatch           " matching braces
set incsearch           " search while typing
set hlsearch            " highlight while searching
set ignorecase          " case insensitive searching
set smartcase           " case sensitive if search has uppercase
set list                " make whitespace visable
set listchars=trail:•   " set trailing spaces
set splitbelow          " split horizontal below
set nobackup            " disable backup files
set noswapfile          " disable swap files

" Normal mode maps
nmap j gj
nmap k gk
nmap Y y$
nmap f<space> :Files<space><return>
nmap K <Nop>
nmap <C-k> :move-2<return>
nmap <C-j> :move+1<return>
nmap <Left>  :echo "no!"<CR>
nmap <Right> :echo "no!"<CR>
nmap <Up>    :echo "no!"<CR>
nmap <Down>  :echo "no!"<CR>

" insert mode maps
imap jj <esc>
imap kk <esc>
imap kj <esc>
imap jk <esc>

" command line maps
cmap Q q!
cmap rld so $MYVIMRC

" disable auto comment insertion
autocmd FileType * setlocal formatoptions-=ro
