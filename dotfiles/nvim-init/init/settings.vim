syntax on                 " syntax processing
filetype on               " filetype syntax highlighting detection
filetype indent on        " filetupe indentation
filetype plugin on        " filetype plugin detection

set langmap+=чявертъуиопшщасдфгхйклзьцжбнмЧЯВЕРТЪУИОПШЩАСДФГХЙКЛЗѝЦЖБНМ;`qwertyuiop[]asdfghjklzxcvbnm~QWERTYUIOP{}ASDFGHJKLZXCVBNM,ю\\,Ю\|,

set clipboard=unnamedplus     " using system clipboard
set termguicolors             " more colors
set background=dark           " set background theme to dark
set relativenumber            " show line nubers relative to the cursor
set number                    " current line number for relative numbers
"set scrolloff=4               " show first/last lines when scrolling
set tabstop=2                 " spaces per tab
set softtabstop=2             " in edit mode
set shiftwidth=2              " width for autoindents
set expandtab                 " tabs to spaces
set showcmd                   " commands in bottom bar
set wildmenu                  " autocomplete for command menu
set showmatch                 " matching braces
set incsearch                 " search while typing
set hlsearch                  " highlight while searching
set ignorecase                " case insensitive searching
set smartcase                 " case sensitive if search has uppercase
set list                      " make whitespace visable
set listchars=trail:•         " set trailing spaces
set nobackup                  " disable backup files
set updatetime=300            " use shorter update time (default 4k)
set splitright                " open vertical splits on the right side
set splitbelow                " split horizontal below
set noswapfile                " disable swap files
set cursorline                " highlight current cursorline
set autoindent                " indent a new line the same amount as the line just typed
set wildmode=longest,list     " get bash-like tab completions
set mouse=a                   " enable mouse support
set hidden                    " Keep unsaved changes in closed buffers
set encoding=utf-8            " The encoding in which files are displayed
set fileencoding=utf-8        " The encoding in which files are saved
set noshowmode                " Don't show the current mode as -- <mode> --

" Disable newline continuation of comments for all sessions
autocmd FileType * setlocal formatoptions-=cro

" set explicit filetype for .pl
au BufNewFile,BufRead *.pl setf prolog

" Disable insertion of matching quote in scheme
autocmd FileType scheme let b:coc_pairs_disabled = ["'"]

" Disable insertion of matching <> in tex
autocmd FileType tex let b:coc_pairs_disabled = ["<"]
