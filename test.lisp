; Defines a function that prints a value...
(define add2 (lambda n (+ n 2)))

(define add4
  (lambda n (add2 (add2 n))))

(print (string (add2 (add2 1))))
(print (string (add4 1)))
