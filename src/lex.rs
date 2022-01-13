use std::vec::Vec;
use std::result::Result;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use crate::syntax::Expr;
use crate::syntax::Atom;

/**
 * Returns all tokens in code separated into a list of strings.
 *
 * @param code Text that represents program.
 * @return All tokens in text separated into a list of strings.
 */
pub fn lexical_analysis(code: &String) -> Result<Vec<String>, String> {
    let is_matching = parens_match_and_exist(code);

    match is_matching {
        true => Result::Ok(split_with_parens(code)),
        false => Result::Err("Missing closing or opening parenthesis.".to_string())
    }
}

/**
 * Parses string that represents non list value into a typed expression.
 *
 * @param atom Textual element of code that is not a list.
 * @return Typed expression object containing value or symbolic name.
 */
fn parse_atom(atom: &String) -> Expr {
    if is_string(atom) {
        return Expr::Atom(Atom::StringLiteral(atom.to_owned()));
    }
    else if BigDecimal::from_str(&atom).is_ok() {
        return Expr::Atom(Atom::Number(BigDecimal::from_str(&atom).unwrap()));
    }
    else if is_boolean(atom) {
        return Expr::Atom(Atom::Boolean(atom == "#t"));
    }
    else if atom == "nil" {
        return Expr::Atom(Atom::Nil);
    }
    else {
        return Expr::Atom(Atom::Symbol(atom.to_owned()));
    }
}

fn parens_match_and_exist(code: &String) -> bool {
    let mut unclosed_opens = 0;

    for c in code.chars() {
        if c == '(' {
            unclosed_opens += 1;
        }
        else if c == ')' {
            unclosed_opens -= 1;
        }
    }

    return unclosed_opens == 0;
}

fn tokens_match_parens(tokens: &Vec<String>) -> bool {
    let mut unclosed_opens = 0;

    for token in tokens {
        if token == "(" {
            unclosed_opens += 1;
        }
        else if token == ")" {
            unclosed_opens -= 1;
        }
    }

    return unclosed_opens == 0;
}

pub fn parse_tokens(tokens: &Vec<String>) -> Expr {
    // recur for interior levels of expression
    // then add them to the expression list.
    let mut local_expr: Vec<Expr> = Vec::new();
    let mut current_expr: Vec<String> = Vec::new();
    let token_len = tokens.len();

    for (i, token) in tokens.iter().enumerate() {
        // check if end or start token.
        if i != 0 && i != token_len {
            // If parens are equally weighted append new expression into
            if tokens_match_parens(&current_expr) && current_expr.len() > 0 {
                let is_list = current_expr.len() > 1;

                if is_list {
                    // Add expression if it has been closed.
                    let expr = parse_tokens(&current_expr);
                    local_expr.push(expr);

                    current_expr.clear();
                }
                else if current_expr.len() == 1 {
                    let expr = parse_atom(&current_expr[0]);
                    local_expr.push(expr);

                    current_expr.clear();
                }
            }

            current_expr.push(token.to_owned());
        }
    }

    if local_expr.len() > 1 {
        return Expr::List(local_expr);
    }
    else {
        return local_expr.get(0).unwrap_or(&Expr::Atom(Atom::Nil)).to_owned();
    }
}

fn is_boolean(atom: &String) -> bool {
    // Boolean literals are '#f' and '#t'
    // for true and false respectively.
    let is_boolean = atom == "#f" ||  atom == "#t";

    return is_boolean;
}

fn is_string(atom: &String) -> bool {
    const BACKSLASH_C: char = '\\'; 
    const QUOTE_C: char = '"';
    let mut is_string: bool = true;

    for (i, c) in atom.chars().enumerate() {
        if i == 0 || i == atom.len() - 1 {
            if c != QUOTE_C {
                is_string = false;
                break;
            }
        }
        else if c == QUOTE_C {
            if i == 1 {
                is_string = false;
                break;
            }
            else {
                let next_c = atom.as_bytes()[i - 1] as char;
                if next_c != BACKSLASH_C {
                    is_string = false;
                    break;
                }
            }
        }
    }

    return is_string;
}

/**
 * Splits syntax into individual tokens.
 * (+ 1 (* 1 2)) => ['(', '+', '(', '*', '1', '2', ')', ')']
 */
fn split_with_parens(code: &String) -> Vec<String> {
    let s0 = code.replace(")", " ) ");
    let s1 = s0.replace("(", " ( ");
    let tokens = s1.split(' ');

    let mut result: Vec<String> = tokens
        .map(|x| x.trim().to_string())
        .filter(|x| !x.trim().is_empty())
        .collect();
    
    // If it's a single token, surround it with parentheses.
    if result.len() == 1 {
        result.insert(0, "(".to_string());
        result.push(")".to_string());
    }

    return result;
}
