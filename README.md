# Mini Lisp Repl

## Intro

## Goals

To implement a minimal Lisp Read-Eval-Print-Loop. There are only a few
keywords used in this implementation.

* lambda
* quote
* =
* car
* cdr
* if
* and
* or
* not

Along with these keywords I am adding a few elementary operators for use with
mathematical programs.

* \+
* \-
* \*
* \/
* %
* \>
* <

## Results

Example REPL Session:

```
> (define n 1)
n
> (define add2 (lambda m (+ m 2)))
add2
> (add2 n)
3
> (define list (quote (5 4 3 2 1)))
list
> list
(5 4 3 2 1)
> (car list)
5
> (cdr list)
(4 3 2 1)
```
