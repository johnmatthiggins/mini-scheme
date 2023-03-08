use crate::env::{Env, Eval};
use crate::syntax;
use std::collections::HashMap;

#[test]
fn one_plus_one_equals_2() {
    let mut environment: Env = Box::new(HashMap::new());
    let lisp = String::from("(+ 1 1)");
    let expected = Ok(String::from("2"));
    let actual = environment
        .eval(&lisp).map(|x| syntax::print_tree(&x, &false));

    assert_eq!(expected, actual);
}

#[test]
fn a_plus_b_equals_three() {
    let mut environment: Env = Box::new(HashMap::new());
    let expected = Ok(String::from("3"));
    let define_a = String::from("(define a 2)");
    let define_b = String::from("(define b 1)");
    let equation = String::from("(+ a b)");

    environment.eval(&define_a);
    environment.eval(&define_b);

    let actual = environment
        .eval(&equation)
        .map(|x| syntax::print_tree(&x, &false));

    assert_eq!(expected, actual);
}

#[test]
fn using_add2_on_3_will_equal_5() {
    let mut environment: Env = Box::new(HashMap::new());
    let expected = Ok(String::from("5"));
    let define_add2 = String::from("(define add2 (lambda n (+ n 2)))");
    let equation = String::from("(add2 3)");

    environment.eval(&define_add2);

    let actual = environment
        .eval(&equation)
        .map(|x| syntax::print_tree(&x, &false));

    assert_eq!(expected, actual);
}

// Overflows stack currently.
#[test]
fn define_add4_in_terms_of_add2() {
    let mut environment: Env = Box::new(HashMap::new());
    let expected = Ok(String::from("5"));
    let define_add2 = String::from("(define add2 (lambda n (+ n 2)))");
    let define_add4 = String::from("(define add4 (lambda n (add2 (add2 n))))");
    let equation = String::from("(add4 1)");

    environment.eval(&define_add2);
    environment.eval(&define_add4);

    let actual = environment
        .eval(&equation)
        .map(|x| syntax::print_tree(&x, &false));

    assert_eq!(expected, actual);
}
