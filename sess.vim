let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/CodeProject/rust/ratatui_playground
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +44 al_pacseek/src/main.rs
badd +158 al_pacseek/src/ui/mod.rs
badd +82 al_pacseek/src/objects/stat.rs
badd +65 al_pacseek/src/backend/mod.rs
badd +53 al_pacseek/src/backend/aur.rs
badd +911 term://~/CodeProject/rust/ratatui_playground//1428265:/bin/zsh
badd +5 ~/.config/nvim/lua/config/plugins/ui/noice.lua
argglobal
%argdel
edit al_pacseek/src/ui/mod.rs
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
balt al_pacseek/src/objects/stat.rs
setlocal foldmethod=manual
setlocal foldexpr=v:lua.vim.treesitter.foldexpr()
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=99
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
sil! 12,14fold
sil! 4,15fold
sil! 3,18fold
sil! 22,23fold
sil! 42,47fold
sil! 48,50fold
sil! 63,70fold
sil! 59,70fold
sil! 59,71fold
sil! 56,71fold
sil! 56,72fold
sil! 75,80fold
sil! 86,90fold
sil! 93,97fold
sil! 101,105fold
sil! 114,116fold
sil! 110,118fold
sil! 25,119fold
sil! 123,125fold
sil! 141,142fold
sil! 132,144fold
sil! 131,145fold
sil! 128,152fold
sil! 127,152fold
sil! 154,160fold
sil! 153,163fold
sil! 122,164fold
sil! 166,168fold
sil! 170,172fold
sil! 174,176fold
sil! 178,180fold
sil! 182,184fold
sil! 189,195fold
sil! 197,200fold
sil! 187,200fold
sil! 201,202fold
sil! 186,215fold
sil! 218,227fold
sil! 216,228fold
sil! 230,233fold
sil! 235,238fold
sil! 240,242fold
sil! 244,248fold
sil! 250,256fold
sil! 121,257fold
let &fdl = &fdl
let s:l = 158 - ((27 * winheight(0) + 24) / 48)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 158
normal! 035|
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
