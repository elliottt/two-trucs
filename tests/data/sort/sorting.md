
# List

## First

This illustrates how `C` will bubble up to the next day, because it contains a
sub-list with unfinished tasks.

* [ ] A
  * [ ] X
* [x] B
* [x] C
  * [ ] Y

# Other List

## Second

This illustrates how all unfinished items are bubbled-up during `next`.

* [ ] 2
* [x] 1

# Bugs

This shows how empty list headings aren't removed when all list items get moved

## Empty Header

* [ ] Bug case
