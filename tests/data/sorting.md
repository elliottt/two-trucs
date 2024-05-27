
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

* [x] 1
* [ ] 2

## Third

Here's a second todo list

* [x] A
* [ ] B

# Bugs

This shows how incomplete items in other H1 sections will all migrate into the
into the H1 section of the next day

## Empty Header

* [ ] Bug case

## Second List

* [ ] Bug case

