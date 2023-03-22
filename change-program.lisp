(define amount 10.26)

(define count-quarters
  (lambda (money) (truncate (/ money 0.25))))

(define count-dollars
  (lambda (money) (truncate money)))

(define count-pennies
  (lambda (money) (truncate (/ money 0.01))))

(define dollars-remaining (count-dollars amount))

(define quarters-remaining (count-quarters (- amount dollars-remaining)))

(define pennies-remaining
  (count-pennies (-
                   (- amount dollars-remaining)
                   (* quarters-remaining 0.25))))

(print (string dollars-remaining))
(print (string quarters-remaining))
(print (string pennies-remaining))
