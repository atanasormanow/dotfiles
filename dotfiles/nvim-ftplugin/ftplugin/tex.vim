" TODO: Configure in Lua
"
" This is necessary for VimTeX to load properly. The "indent" is optional.
" Note that most plugin managers will do this automatically.
filetype plugin indent on

" This enables Vim's and neovim's syntax-related features. Without this, some
" VimTeX features will not work (see ":help vimtex-requirements" for more
" info).
syntax enable

" Conceal command with unicode when the cursor is not on the same line
set textwidth=0
set conceallevel=1

" Use latex flavor
let g:tex_flavor='latex'

" Never open the quickfix window automatically
let g:vimtex_quickfix_mode=0

" conceal all except subscripts
let g:tex_conceal='abdmg'

" Viewer options: One may configure the viewer either by specifying a built-in
" viewer method:
let g:vimtex_view_method = 'zathura'

" Don't automatically open PDF viewer after first compilation
let g:vimtex_view_automatic = 0

" Most VimTeX mappings rely on localleader and this can be changed with the
" following line. The default is usually fine and is the symbol "\".
let maplocalleader = ","

" Get Vim's window ID for switching focus from Zathura to Vim using xdotool.
" Only set this variable once for the current Vim instance.
if !exists("g:vim_window_id")
  let g:vim_window_id = system("xdotool getactivewindow")
endif

function! s:TexFocusVim() abort
  " Give window manager time to recognize focus moved to Zathura;
  " tweak the 200m as needed for your hardware and window manager.
  sleep 200m

  " Refocus Vim and redraw the screen
  silent execute "!xdotool windowfocus " . expand(g:vim_window_id)
  redraw!
endfunction

augroup vimtex_event_focus
  au!
  au User VimtexEventView call s:TexFocusVim()
augroup END

" MAPPINGS:
nnoremap <localleader>ss <plug>(vimtex-compile-ss)
nnoremap <localleader>f <cmd>VimtexView<cr>
nnoremap <localleader>i <cmd>VimtexInfo<cr>

" Auto insert matching $
let b:coc_pairs = [["$","$"]]

" TODO: Enable which key
