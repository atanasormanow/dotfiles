" Show leader commands with space as leader
nnoremap <silent> <leader> :WhichKey '<Space>'<CR>
vnoremap <silent> <leader> :WhichKeyVisual '<Space>'<CR>

" By default timeoutlen is 1000 ms
set timeoutlen=600

" Cover the line numbers
let g:which_key_disable_default_offset=1

" Use split instead of floating window
let g:which_key_use_floating_win=0

" Show items only from which_key_map
let g:which_key_ignore_outside_mappings=1

" Hide status line when showing WhichKey
autocmd! FileType which_key
autocmd  FileType which_key set laststatus=0 noshowmode noruler
  \| autocmd BufLeave <buffer> set laststatus=2 showmode ruler

" Dictionaries
call which_key#register('<Space>', "g:which_key_map", 'n')
call which_key#register('<Space>', "g:which_key_map_visual", 'v')

let g:which_key_map_visual = {}
let g:which_key_map = {}

let g:which_key_map.b = 'open buffer'
let g:which_key_map.f = 'open file'
let g:which_key_map.l = 'split right'
let g:which_key_map.j = 'split bottom'
let g:which_key_map.t = 'new tab'
let g:which_key_map.r = 'source VIMRC'
let g:which_key_map.m = 'jump to mark'
let g:which_key_map.n = 'no highlight'
let g:which_key_map.F = 'open file in new tab'
let g:which_key_map['='] = 'autoformat file'
let g:which_key_map['<return>'] = 'open terminal'

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

let g:which_key_map_visual.c = which_key_map.c
