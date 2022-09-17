echo "Hello from keymaps!"
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
