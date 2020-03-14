
let s:root = expand('<sfile>:p:h:h')
let s:bin = s:root."/bin/two-trucs"

function! two_trucs#sort()
    let saved = getcurpos()
    execute ":%!".s:bin
    call setpos('.', l:saved)
endfunction

function! two_trucs#next(...)
    if a:0 == 0
        let title = strftime("%Y-%m-%d")
    else
        let title = a:1
    endif

    let saved = getcurpos()
    execute ":%!".s:bin." -n -t '".l:title."'"
    call setpos('.', l:saved)
endfunction
