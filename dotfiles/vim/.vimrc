" Auto-install vim-plug (--insecure if using proxy)
if empty(glob('~/.vim/autoload/plug.vim'))
    silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
        \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
    autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Map Bulgarian keychords to English ones in vim.
" NOTE: compound maps won't work
set langmap+=чявертъуиопшщасдфгхйклзьцжбнмЧЯВЕРТЪУИОПШЩАСДФГХЙКЛЗѝЦЖБНМ;`qwertyuiop[]asdfghjklzxcvbnm~QWERTYUIOP{}ASDFGHJKLZXCVBNM,ю\\,Ю\|,

" Options
let mapleader=","       " set the leader key
syntax enable           " syntax processing
filetype on             " filetype syntax highlighting detection
filetype indent on      " filetupe indentation
filetype plugin on      " filetype plugin detection
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
nnoremap <leader><return> :terminal ++rows=12<return>
nnoremap <leader>t :tabnew<return>
nnoremap <leader>f :Files<space>
nnoremap <leader>F :tabnew<return>:Files<space>
nnoremap <leader>b :Buffers<return>
nnoremap <leader>w 5<C-w>-
nnoremap <leader>s 5<C-w>+
nnoremap <leader>a 10<C-w><
nnoremap <leader>d 10<C-w>>
nnoremap <leader>n :noh<CR>
nnoremap <leader>. `
nnoremap <leader><space> :%s/\s\+$//e<CR>
nnoremap j gj
nnoremap k gk
nnoremap Y y$
nnoremap K <Nop>
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l
nnoremap :W :w
nnoremap :в :w
nnoremap :я :q
nnoremap :Я :q!
nnoremap :вя :wq

" insert mode maps
inoremap jj <esc>
inoremap kj <esc>
inoremap jk <esc>
inoremap йй <esc>
inoremap кк <esc>
inoremap кй <esc>
inoremap <C-v> <C-R>"

" command line maps
cnoremap Q q!
cnoremap rld source $MYVIMRC

" visual line maps
vnoremap P "0p

" Disable Polyglot for:
" let g:polyglot_disabled = ['language_pack_here']

" TODO current version 8.1.0
"coc.nvim works best on vim >= 8.1.1719 and neovim >= 0.4.0, consider upgrade your vim.
"You can add this to your vimrc to avoid this message:
let g:coc_disable_startup_warning = 1
"Note that some features may behave incorrectly.

" Plugins
call plug#begin('~/.vim/plugged')

" Color theme
Plug 'morhetz/gruvbox'

" Fuzzy search for files
Plug 'junegunn/fzf', { 'do': './install --bin' }
Plug 'junegunn/fzf.vim'

" Status bar
Plug 'vim-airline/vim-airline'

" Use tab for autocompletion
Plug 'ervandew/supertab'

" Basic syntax highlighting
Plug 'sheerun/vim-polyglot'

" Open and serve markdown in real time
Plug 'iamcco/markdown-preview.nvim', { 'do': { -> mkdp#util#install() } }

" Coc stuff
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'elixir-lsp/coc-elixir', {'do': 'yarn install && yarn prepack'}

" Use git in vim
Plug 'tpope/vim-fugitive'

" Use automatic vim sessions
Plug 'thaerkh/vim-workspace'

" Use code search and view
Plug 'dyng/ctrlsf.vim'

" TODO maybe use some day if needed
" Plug 'Quramy/tsuquyomi'
" Plug 'prettier/vim-prettier', {
"   \ 'do': 'yarn install',
"   \ 'for': ['javascript', 'typescript', 'css', 'json', 'markdown', 'yaml', 'html'] }

call plug#end()

" Plugins configuration
colorscheme gruvbox
let g:gruvbox_contrast_light = 'hard'
let g:gruvbox_contrast_dark = 'soft'

" Displayu tabs at the top
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#formatter = 'unique_tail'

" disable auto comment insertion
autocmd FileType * setlocal formatoptions-=ro
