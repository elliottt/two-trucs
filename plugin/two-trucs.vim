" Sort the todo items present and re-render the markdown file to the same
" buffer.
command! -nargs=0 TTSort call two_trucs#sort()

" Collect all outstanding todo items, and move them into a new top-level
" heading. By default the name of the heading is the same as today's date, but
" you can supply an argument that is an alternate title.
command! -nargs=? TTNext call two_trucs#next(<args>)
