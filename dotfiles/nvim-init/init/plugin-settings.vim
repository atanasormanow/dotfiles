" Set colorscheme before anything else gruvbox related
colorscheme gruvbox

" Set gruvbox colorscheme contrast
let g:gruvbox_contrast_light='hard'
let g:gruvbox_contrast_dark='soft'

" include hidden files, but ignore .git and other
let $FZF_DEFAULT_COMMAND='find . \! \( -type d -path ./.git -prune \) \! -type d \! -name ''*.tags'' -printf ''%P\n'''

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

" Enable trimming of trailing whitespace when uncommenting
let g:NERDTrimTrailingWhitespace=1

" Auto create session on save
let g:auto_session_create_enabled='false'
