(define fac
    (lambda (n)
        (if (= n 0)
            1
            (* n (fac (- n 1)))
        )))
