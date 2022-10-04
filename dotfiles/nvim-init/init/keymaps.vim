" NORMAL MAPS:
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


" INSERT MAPS:
""""""""""""""
" easier insert mode
inoremap kk <esc>
inoremap jj <esc>
inoremap kj <esc>
inoremap jk <esc>


" TERMINAL MAPS:
""""""""""""""""
" normal mode
tnoremap <C-n> <C-\><C-n>


" EVAL MAPS:
""""""""""""
" Use nmap/imap/vmap if you want the right side to evaluate
" (this allows recursive mappings)
"
" search for the word under the cursor with CtrlSF
nmap <C-F> <Plug>CtrlSFCwordPath<return>
" show diagnostics in a horizontal split
nmap K <cmd>CocDiagnostics<return>
" Use <C-j> for both expand and jump (make expand higher priority.)
imap <C-j> <Plug>(coc-snippets-expand-jump)

" COMMAND_LINE MAPS:
""""""""""""""""""""
" Copy current working directory
cnoremap cpd ! pwd \| xclip
