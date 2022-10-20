" set the leader key
let g:mapleader=" "

" Show leader commands menu
nnoremap <silent> <leader> :WhichKey '<Space>'<return>
vnoremap <silent> <leader> :WhichKeyVisual '<Space>'<return>

" Set timeout before opening which_key (default: 1000)
set timeoutlen=600

" Cover the line numbers
let g:which_key_disable_default_offset=1

" Use split instead of floating window
let g:which_key_use_floating_win=0

" Show only described (below) leader commands
let g:which_key_ignore_outside_mappings=1

" Hide status line when showing WhichKey
autocmd! FileType which_key
autocmd  FileType which_key set laststatus=0 noshowmode noruler
  \| autocmd BufLeave <buffer> set laststatus=2 showmode ruler

" Use command description dictionaries
call which_key#register('<Space>', "g:which_key_map", 'n')
call which_key#register('<Space>', "g:which_key_map_visual", 'v')


" NORMAL MAPS:
""""""""""""""
let g:which_key_map = {}

" Prefix entries
let g:which_key_map.s = { 'name' : '+session' }

nnoremap <leader>b <cmd>Buffers<return>
let g:which_key_map.b = 'open buffer'

nnoremap <leader>f <cmd>Files<return>
let g:which_key_map.f = 'open file'

nnoremap <leader>l <cmd>vsplit<return>
let g:which_key_map.l = 'split right'

nnoremap <leader>j <cmd>split<return>
let g:which_key_map.j = 'split bottom'

nnoremap <leader>t <cmd>tabnew<return>
let g:which_key_map.t = 'new tab'

nnoremap <leader>r <cmd>source $MYVIMRC<return>
let g:which_key_map.r = 'source VIMRC'

nnoremap <leader>m <cmd>Marks<return>
let g:which_key_map.m = 'fzf marks menu'

nnoremap <leader>n <cmd>noh<return>
let g:which_key_map.n = 'no highlight'

nnoremap <leader>e <cmd>CocCommand snippets.editSnippets<return>
let g:which_key_map.e = 'edit filetype snippets'

nnoremap <leader>F <cmd>tabnew<return><cmd>Files<return>
let g:which_key_map.F = 'open file in new tab'

nmap <leader>= <Plug>(coc-format)
let g:which_key_map['='] = 'autoformat file'

nnoremap <leader><return> <cmd>split \| resize -5 \| terminal<return>
let g:which_key_map['<return>'] = 'open terminal'

nnoremap <leader><tab> <C-^>
let g:which_key_map['<tab>'] = 'last used buffer'

nnoremap <leader>ss <cmd>SaveSession<return>
let g:which_key_map.s.s = 'save'

nnoremap <leader>sd <cmd>DeleteSession<return>
let g:which_key_map.s.d = 'delete'

nnoremap <leader>sl <cmd>RestoreSession<return>
let g:which_key_map.s.l = 'restore'

" Leader commands from NERDcommenter
let g:which_key_map.c = {
      \ 'name' : '+commenting',
      \ 'SPC'  : 'toggle comment',
      \ 'c'    : 'comment',
      \ 'i'    : 'invert comments',
      \ 's'    : 'comment fancy',
      \ 'a'    : 'toggle alternative delimiter',
      \ 'u'    : 'uncomment',
      \ 'A'    : 'append comment',
      \ 'y'    : 'yank & comment',
      \}


" VISUAL MAPS:
""""""""""""""
let g:which_key_map_visual = {}
let g:which_key_map_visual.c = which_key_map.c

xnoremap <leader>p "_dP
let g:which_key_map_visual.p = 'paste (no buffer overwrite)'

