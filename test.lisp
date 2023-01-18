; Defines a function that prints a value...
(define print-file (lambda file (print (slurp 'Cargo.toml'))))

(define add2 (lambda n (+ n 2)))

(print-file 'Cargo.toml')

(print 'Hello World!')
