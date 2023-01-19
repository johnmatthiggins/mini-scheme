; fibonacii sequence
(define fib 
  (lambda (n)
    (if (or (= n 2) (= n 1))
      1
      (+ (fib (- n 1)) (fib (- n 2)))
    )))
(fib 5)
