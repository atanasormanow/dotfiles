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
set termguicolors       " more colors
set background=dark     " set background theme to dark
set number              " current line number for relative numbers
set scrolloff=2         " show first/last lines when scrolling
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
nnoremap <leader>f :Files ~<return>
nnoremap <leader>t :terminal<return>
nnoremap <leader>b :Buffers<return>
nnoremap <leader>w <C-w>-
nnoremap <leader>s <C-w>+
nnoremap K <Nop>
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l
nnoremap <Left>  :echo "no!"<CR>
nnoremap <Right> :echo "no!"<CR>
nnoremap <Up>    :echo "no!"<CR>
nnoremap <Down>  :echo "no!"<CR>

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
Plug 'iamcco/markdown-preview.nvim', { 'do': { -> mkdp#util#install() } }
Plug 'sheerun/vim-polyglot'
call plug#end()

" Plugins configuration
colorscheme gruvbox
let g:gruvbox_contrast_light = 'hard'
let g:gruvbox_contrast_dark = 'hard'

"let g:airline_powerline_fonts = 1
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#left_alt_sep = ' | '

let g:mkdp_markdown_css = '/home/nakata/Downloads/github_css/github.css'

" disable auto comment insertion
autocmd FileType * setlocal formatoptions-=ro
