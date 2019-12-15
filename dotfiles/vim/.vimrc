" Auto-install vim-plug (--insecure if using proxy)
if empty(glob('~/.vim/autoload/plug.vim'))
    silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
        \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
    autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Options
let mapleader=","       " set the leader key
syntax enable           " syntax processing
filetype on             " filetype syntax highlighting detection
filetype plugin on      " filetype plugin detection
filetype indent on      " filetype indent detection
set clipboard=unnamed   " read from the system register
set termguicolors       " more colors
set background=dark     " set background theme to dark
set number              " current line number for relative numbers
set scrolloff=4         " show first/last lines when scrolling
set tabstop=2           " spaces per tab
set softtabstop=2       " in edit mode
set shiftwidth=0        " use tabstop averywhere
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
nnoremap j gj
nnoremap k gk
nnoremap Y y$
nnoremap <leader>f :Files<space>
nnoremap <leader><return> :terminal ++rows=12<return>
nnoremap <leader>t :tabnew<return>
nnoremap <leader>b :Buffers<return>
nnoremap <leader>w 5<C-w>-
nnoremap <leader>s 5<C-w>+
nnoremap K <Nop>
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l
nnoremap <Left>  :echo "no!"<CR>
nnoremap <Right> :echo "no!"<CR>
nnoremap <Up>    :echo "no!"<CR>
nnoremap <Down>  :echo "no!"<CR>
nnoremap :W :w

" insert mode maps
inoremap jj <esc>
inoremap kk <esc>
inoremap kj <esc>
inoremap jk <esc>
inoremap <C-v> <C-R>"

" command line maps
cnoremap Q q!
cnoremap rld source $MYVIMRC

" Plugins
call plug#begin('~/.vim/plugged')
Plug 'junegunn/fzf', { 'do': './install --bin' }
Plug 'junegunn/fzf.vim'
Plug 'ervandew/supertab'
Plug 'morhetz/gruvbox'
Plug 'vim-airline/vim-airline'
Plug 'iamcco/markdown-preview.nvim'
"Plug 'sheerun/vim-polyglot'
call plug#end()

" Plugins configuration
colorscheme gruvbox
let g:gruvbox_contrast_light = 'hard'
let g:gruvbox_contrast_dark = 'soft'

let g:airline#extensions#tabline#enabled = 1

" disable auto comment insertion
autocmd FileType * setlocal formatoptions-=ro
