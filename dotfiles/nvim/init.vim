" OPTIONS:
""""""""""
let mapleader=","         " set the leader key
syntax on                 " syntax processing
filetype on               " filetype syntax highlighting detection
filetype indent on        " filetupe indentation
filetype plugin on        " filetype plugin detection

set langmap+=чявертъуиопшщасдфгхйклзьцжбнмЧЯВЕРТЪУИОПШЩАСДФГХЙКЛЗѝЦЖБНМ;`qwertyuiop[]asdfghjklzxcvbnm~QWERTYUIOP{}ASDFGHJKLZXCVBNM,ю\\,Ю\|,

set nocompatible          " disable vi compatibility
set clipboard=unnamedplus " using system clipboard
set termguicolors         " more colors
set background=dark       " set background theme to dark
set number                " current line number for relative numbers
set scrolloff=4           " show first/last lines when scrolling
set tabstop=2             " spaces per tab
set softtabstop=2         " in edit mode
set shiftwidth=2          " width for autoindents
set expandtab             " tabs to spaces
set showcmd               " commands in bottom bar
set wildmenu              " autocomplete for command menu
set showmatch             " matching braces
set incsearch             " search while typing
set hlsearch              " highlight while searching
set ignorecase          " case insensitive searching
set smartcase             " case sensitive if search has uppercase
set list                  " make whitespace visable
set listchars=trail:•     " set trailing spaces
set nobackup              " disable backup files
set updatetime=300        " use shorter update time (default 4k)
set splitright            " open vertical splits on the right side
set splitbelow            " split horizontal below
set noswapfile            " disable swap files
set cursorline            " highlight current cursorline
set autoindent            " indent a new line the same amount as the line just typed
set wildmode=longest,list " get bash-like tab completions
set ttyfast               " Speed up scrolling in Vim
"set spell                 " enable spell check (may need to download language package)


" NORMAL_MAPS:
""""""""""""""
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
" shorter commands for cl
nnoremap :W :w
nnoremap :Q :q!
" resize splits
nnoremap <up>    5<C-w>-
nnoremap <down>  5<C-w>+
nnoremap <left>  10<C-w><
nnoremap <right> 10<C-w>>
" ??? open last closed tab
" nnoremap <C-T> :tabnew#<return>
" open terminal in smaller bottom split
nmap <leader><return> :split \| resize -5 \| terminal<return>
" go to mark
nnoremap <leader>. `
" remove trailing spaces
nnoremap <leader><space> :%s/\s\+$//e<CR>
" no highlight
nnoremap <leader>n :noh<CR>
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
" open vertical split
nnoremap <leader>v :vsplit<return>
" search with ctrlsf
noremap <leader>/ :CtrlSF<space>
" reload configuration file
noremap <leader>r :source $MYVIMRC<return>


" INSERT_MAPS:
""""""""""""""
" easier insert mode
inoremap jj <esc>
inoremap kj <esc>
inoremap jk <esc>


" VISUAL_MAPS:
""""""""""""""
" ??? paste yanked word instead of last deleted one
" NOTE: when doing consecutive pastes like this,
" you must start with a yank instead of a delete
" vnoremap P "0p


" EVAL_MAPS:
""""""""""""
" Use nmap/imap/vmap if you want the right side to evaluate
" (allows recursive mappings)
nmap <C-I> <Plug>(coc-format)
nmap <C-f> <Plug>CtrlSFCwordPath<return>
" show diagnostics in a horizontal split
nmap K :CocDiagnostics<return>
" open file with fzf in a new tab
nmap <leader>F ,t,f
" search for visually selected text
vmap F y/<C-r>"<return>
" ??? yank then delete the word on the cursor
" the reason for this is to have the word in the yank register,
" so you can do consecutive pastes with P in visual mode
" nmap <leader>d yawdaw


" OTHER:
""""""""
" disable auto comment insertion on return
autocmd FileType * setlocal formatoptions-=ro
au BufNewFile,BufRead *.pl setf prolog
tnoremap <C-n> <C-\><C-n>


" PLUGINS:
""""""""""
call plug#begin('~/.vim/plugged')
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
call plug#end()


" PLUGIN_RELATED:
"""""""""""""""""
" include hidden files, but ignore .git and other
let $FZF_DEFAULT_COMMAND='find . \! \( -type d -path ./.git -prune \) \! -type d \! -name ''*.tags'' -printf ''%P\n'''

" Set colorscheme before anything else gruvbox related
colorscheme gruvbox

" Set gruvbox colorscheme contrast
let g:gruvbox_contrast_light = 'hard'
let g:gruvbox_contrast_dark = 'soft'

" Traverse completion list top-down
let g:SuperTabDefaultCompletionType = "<c-n>"

" open search window on the right
let g:ctrlsf_position = 'right'

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


" NOTES:
""""""""
" - maybe use lualine/feline as a status bar
" - setup bg lang maps
