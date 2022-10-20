" Set colorscheme before anything else gruvbox related
colorscheme gruvbox

" Set gruvbox colorscheme contrast
let g:gruvbox_contrast_light='hard'
let g:gruvbox_contrast_dark='soft'

" include hidden files, but ignore .git and other
let $FZF_DEFAULT_COMMAND='find . \! \( -type d -path ./.git -prune \) \! -type d \! -name ''*.tags'' -printf ''%P\n'''

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

" Enable trimming of trailing whitespace when uncommenting
let g:NERDTrimTrailingWhitespace=1

" Auto create session on save
let g:auto_session_create_enabled='false'

" Enable rainbow parentheses
" set to 0 if you want to enable it later via :RainbowToggle
let g:rainbow_active = 1

" Highlight the word under cursor
let g:vim_current_word#highlight_current_word = 0

" Syntax highlighting options for Haskell:
" enable highlighting of `forall`
let g:haskell_enable_quantification = 1
" enable highlighting of `mdo` and `rec`
let g:haskell_enable_recursivedo = 1
" enable highlighting of `proc`
let g:haskell_enable_arrowsyntax = 1
" enable highlighting of `pattern`
let g:haskell_enable_pattern_synonyms = 1
" enable highlighting of type roles
let g:haskell_enable_typeroles = 1
" enable highlighting of `static`
let g:haskell_enable_static_pointers = 1
" enable highlighting of backpack keywords
let g:haskell_backpack = 1

