" OPTIONS:
""""""""""
let g:mapleader=" "     " set the leader key

syntax on                 " syntax processing
filetype on               " filetype syntax highlighting detection
filetype indent on        " filetupe indentation
filetype plugin on        " filetype plugin detection

set langmap+=чявертъуиопшщасдфгхйклзьцжбнмЧЯВЕРТЪУИОПШЩАСДФГХЙКЛЗѝЦЖБНМ;`qwertyuiop[]asdfghjklzxcvbnm~QWERTYUIOP{}ASDFGHJKLZXCVBNM,ю\\,Ю\|,

set clipboard=unnamedplus     " using system clipboard
set termguicolors               " more colors
set background=dark           " set background theme to dark
set relativenumber              " show line nubers relative to the cursor
set number                      " current line number for relative numbers
set scrolloff=4               " show first/last lines when scrolling
set tabstop=2                 " spaces per tab
set softtabstop=2             " in edit mode
set shiftwidth=2              " width for autoindents
set expandtab                   " tabs to spaces
set showcmd                     " commands in bottom bar
set wildmenu                    " autocomplete for command menu
set showmatch                   " matching braces
set incsearch                   " search while typing
set hlsearch                    " highlight while searching
set ignorecase                  " case insensitive searching
set smartcase                   " case sensitive if search has uppercase
set list                        " make whitespace visable
set listchars=trail:•         " set trailing spaces
set nobackup                    " disable backup files
set updatetime=300            " use shorter update time (default 4k)
set splitright                  " open vertical splits on the right side
set splitbelow                  " split horizontal below
set noswapfile                  " disable swap files
set cursorline                  " highlight current cursorline
set autoindent                  " indent a new line the same amount as the line just typed
set wildmode=longest,list     " get bash-like tab completions
" set mouse=a                 " enable mouse support
set hidden                      " Keep unsaved changes in closed buffers
set encoding=utf-8            " The encoding in which files are displayed
set fileencoding=utf-8        " The encoding in which files are saved
set noshowmode                  " Don't show the current mode as -- <mode> --
set formatoptions-=cro        " Disable newline continuation of comments


" NORMAL_MAPS:
""""""""""""""
" move between visual lines
nnoremap j gj
nnoremap k gk
" move between tabs
nnoremap gj gT
nnoremap gk gt
" move tabs around
nnoremap gJ <cmd>tabmove -1<return>
nnoremap gK <cmd>tabmove +1<return>
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
nnoremap <leader><return> <cmd>split \| resize -5 \| terminal<return>
" go to mark
nnoremap <leader>m `
" no highlight
nnoremap <leader>n <cmd>noh<CR>
" open new tab
" NOTE: maybe change position of the new tab, as the command offers flexibility
nnoremap <leader>t <cmd>tabnew<return>
" open file with fzf
nnoremap <leader>f <cmd>Files<return>
" open file with fzf in a new tab
nnoremap <leader>F <cmd>tabnew<return><cmd>Files<return>
" open file from buffers
nnoremap <leader>b <cmd>Buffers<return>
" open vertical split
nnoremap <leader>l <cmd>vsplit<return>
" open horizontal split
nnoremap <leader>j <cmd>split<return>
" reload configuration file
nnoremap <leader>r <cmd>source $MYVIMRC<return>


" INSERT_MAPS:
""""""""""""""
" easier insert mode
inoremap kk <esc>
inoremap jj <esc>
inoremap kj <esc>
inoremap jk <esc>



" VISUAL_MAPS:
""""""""""""""
" paste without overwriting the paste register
xnoremap <leader>p "_dP


" TERMINAL_MAPS:
""""""""""""""""
" normal mode
tnoremap <C-n> <C-\><C-n>


" EVAL_MAPS:
""""""""""""
" Use nmap/imap/vmap if you want the right side to evaluate
" (allows recursive mappings)
"
" use autoformat (if available)
nmap <leader>= <Plug>(coc-format)
" search for the word under the cursor with CtrlSF
nmap <C-F> <Plug>CtrlSFCwordPath<return>
" show diagnostics in a horizontal split
nmap K <cmd>CocDiagnostics<return>
" Use <C-j> for both expand and jump (make expand higher priority.)
imap <C-j> <Plug>(coc-snippets-expand-jump)

" COMMAND_LINE_MAPS:
""""""""""""""""""""
" Copy current working directory
cnoremap cpd ! pwd \| xclip

" OTHER:
""""""""
" set explicit filetype for .pl
au BufNewFile,BufRead *.pl setf prolog


" PLUGINS:
""""""""""
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


" PLUGIN_RELATED:
"""""""""""""""""
" include hidden files, but ignore .git and other
let $FZF_DEFAULT_COMMAND='find . \! \( -type d -path ./.git -prune \) \! -type d \! -name ''*.tags'' -printf ''%P\n'''

" Set colorscheme before anything else gruvbox related
colorscheme gruvbox

" Set gruvbox colorscheme contrast
let g:gruvbox_contrast_light='hard'
let g:gruvbox_contrast_dark='soft'

" Traverse completion list top-down
let g:SuperTabDefaultCompletionType="<c-n>"

" open search window on the right
let g:ctrlsf_position='right'

" Display tabs at the top
let g:airline#extensions#tabline#enabled=1

" Hide buffer number in tabs
let g:airline#extensions#tabline#show_tab_nr=1

" Display only file name in tabs
let g:airline#extensions#tabline#formatter='unique_tail'

" Display powerline symbols
let g:airline_powerline_fonts=1

" Set tabline label to blank, opposed to default 'tabs'
let g:airline#extensions#tabline#tabs_label=''

" enable vimtex integration
let g:airline#extensions#vimtex#enabled=1

" Don't show splits and 'buffers' label
let airline#extensions#tabline#tabs_label=''
let airline#extensions#tabline#show_splits=0

" Create default mappings
let g:NERDCreateDefaultMappings=1

" Add spaces after comment delimiters by default
let g:NERDSpaceDelims=1

" Align line-wise comment delimiters flush left instead of following code indentation
let g:NERDDefaultAlign='left'

" Auto create session on save
let g:auto_session_create_enabled='false'

" Show leader commands with space as leader
nnoremap <silent> <leader> :WhichKey '<Space>'<CR>

" By default timeoutlen is 1000 ms
set timeoutlen=600

" Cover the line numbers
let g:which_key_disable_default_offset=1

" Use split instead of floating window
let g:which_key_use_floating_win=0

" Hide status line when showing WhichKey
autocmd! FileType which_key
autocmd  FileType which_key set laststatus=0 noshowmode noruler
  \| autocmd BufLeave <buffer> set laststatus=2 showmode ruler


" NOTES:
""""""""
" - migrate config to lua at some point
" - then maybe use lualine/feline as a status bar
