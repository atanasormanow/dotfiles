" Auto-install vim-plug (--insecure if using proxy)
if empty(glob('~/.vim/autoload/plug.vim'))
    silent !curl -fLo ~/.vim/autoload/plug.vim --create-dirs
        \ https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
    autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Map Bulgarian keychords to English ones in vim.
" NOTE: compound maps won't work
set langmap+=чявертъуиопшщасдфгхйклзьцжбнмЧЯВЕРТЪУИОПШЩАСДФГХЙКЛЗѝЦЖБНМ;`qwertyuiop[]asdfghjklzxcvbnm~QWERTYUIOP{}ASDFGHJKLZXCVBNM,ю\\,Ю\|,

" OPTIONS:
let mapleader=","       " set the leader key
syntax enable           " syntax processing
filetype on             " filetype syntax highlighting detection
filetype indent on      " filetupe indentation
filetype plugin on      " filetype plugin detection
set nocompatible        " disable vi compatibility
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
set updatetime=300      " use shorter update time (default 4k)

" NORMAL_MAPS:
" move between visual lines
nnoremap j gj
nnoremap k gk
" move between tabs
nnoremap gj gT
nnoremap gk gt
" move tabs around
nnoremap gJ :tabmove -1<return>
nnoremap gK :tabmove +1<return>
" yank untill the end of the line
nnoremap Y y$
" do not jump to next match
nnoremap * *``
" move between splits
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l
" shorter commands for cl + cyrillic
nnoremap :W :w
nnoremap :Q :q!
nnoremap :в :w
nnoremap :я :q
nnoremap :Я :q!
nnoremap :вя :wq
" resize splits
nnoremap <up>    5<C-w>-
nnoremap <down>  5<C-w>+
nnoremap <left>  10<C-w><
nnoremap <right> 10<C-w>>
" open last closed tab
nnoremap <C-T> :tabnew#<return>
" open terminal in small bottom split
nnoremap <leader><return> :terminal ++rows=12<return>
" go to mark
nnoremap <leader>. `
" remove trailing spaces
nnoremap <leader><space> :%s/\s\+$//e<CR>
" no highlight
nnoremap <leader><esc> :noh<CR>
" open menu with all commands (fzf)
nnoremap <leader>c :Maps<return>
" open new tab
" NOTE: maybe change position of the new tab,
" as the command offers flexibility
nnoremap <leader>t :tabnew<return>
" open file with fzf
nnoremap <leader>f :Files<return>
" open file from buffers
nnoremap <leader>b :Buffers<return>
" search with ctrlsf
noremap <leader>/ :CtrlSF<space>

" Use nmap if you want the right side to evaluate
" (allows recursive mappings as well)
nmap <C-I> <Plug>(coc-format)
" show diagnostics in a horizontal split
nmap K :CocDiagnostics<return>
" open file with fzf in a new tab
nmap <leader>F ,t,f
" open file from buffers in a new tab
nmap <leader>B ,t,b
" open file from buffers in a horizontal split
nmap <leader>v :vsplit<return>,b
" open file with fzf in a horizontal split
nmap <leader>V :vsplit<return>,f
" yank then delete the word on the cursor
" the reason for this is to have the word in the yank register,
" so you can do consecutive pastes with P in visual mode
nmap <leader>d yawdaw

" INSERT_MAPS:
" easier insert mode + cyrillic
inoremap jj <esc>
inoremap kj <esc>
inoremap jk <esc>
inoremap йй <esc>
inoremap кк <esc>
inoremap кй <esc>
" paste from unnamed clipboard in insert mode
inoremap <C-v> <C-R>"

" COMMAND_MAPS:
" reload .vimrc
cnoremap rld source $MYVIMRC

" VISUAL_MAPS:
" paste yanked word instead of last deleted one
" NOTE: when doing consecutive pastes like this,
" you must start with a yank instead of a delete
vnoremap P "0p

" PLUG_PRE_CONFIGS:
" Disable Polyglot for certain languages
" let g:polyglot_disabled = ['language_pack_here']

" TODO current version 8.1.1401
" coc.nvim works best on vim >= 8.1.1719 and neovim >= 0.4.0,
" consider upgrade your vim.
"
" Avoid this message:
let g:coc_disable_startup_warning = 1
" NOTE: Some features may behave incorrectly.

" PLUGINS:
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

call plug#end()


" OTHER:
" Set colorscheme before setting anything else gruvbox related
colorscheme gruvbox

" disable auto comment insertion on return
autocmd FileType * setlocal formatoptions-=ro


" PLUG_POST_CONFIGS:
" Set gruvbox colorscheme contrast
let g:gruvbox_contrast_light = 'hard'
let g:gruvbox_contrast_dark = 'soft'

" Traverse completion list top-down
let g:SuperTabDefaultCompletionType = "<c-n>"

" Display tabs at the top
let g:airline#extensions#tabline#enabled = 1

" Hide buffer number in tabs
let g:airline#extensions#tabline#show_tab_nr = 1

" Display only file name in tabs
let g:airline#extensions#tabline#formatter = 'unique_tail'

" Display powerline symbols
let g:airline_powerline_fonts = 1

" Set tabline label to blank, opposed to default 'tabs'
let g:airline#extensions#tabline#tabs_label = ''

" Don't show splits and 'buffers' label
let airline#extensions#tabline#tabs_label = ''
let airline#extensions#tabline#show_splits = 0

" Save sassions outside of working directory
let g:workspace_session_directory = $HOME . '/.vim/sessions/'

" Disable persistent undo history, as it keeps ./undodir in the workspace folder
let g:workspace_persist_undo_history = 0

" Do not remove trailing spaces on save,
" as they might be needed in a .md file for example
let g:workspace_autosave_untrailspaces = 0

" open search window on the right
let g:ctrlsf_position = 'right'
