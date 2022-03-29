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

### Comparison Operators
```
> (define n 1)
n
> (define m 2)
m
> (= m n)
#f
> (define T #t)
T
> (define T2 #t)
T2
> (= T T2)
#t
> (define l1 (quote (1 2 3 4 5)))
l1
> (define l2 (quote (2 3 4 5 1)))
l2
> (= l2 l1)
#f
> (define l3 (quote (1 2 3 4 5)))
l3
> (= l3 l1)
#t
```

### Mathematical Operations
```
> (define x 2)
x
> (define y 3)
y
> (+ x y)
5
> (* x y)
6
> (/ x y)
0.6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666667
> (- x y)
-1
> (- y x)
1
> (* x y y)
18
```

### List Operations
```
> (quote (1 2 3 4))
(1 2 3 4)
> (define ages (quote (35 67 23 44)))
ages
> (car ages)
35
> (cdr ages)
(67 23 44)
> (car (cdr (ages)))
67
```
