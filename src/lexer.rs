use std::vec::Vec;
use std::result::Result;
use std::collections::LinkedList;

mod lexer {
    /**
     * Returns all tokens in code separated into a list of strings.
     *
     * @param code Text that represents program.
     * @return All tokens in text separated into a list of strings.
     */
    pub fn lexical_analysis(code: &String) -> Result<Vec<String>, String> {
        let is_matching = has_matching_parens(code);

        if is_matching {
            let result = Result::Ok(split_with_parens(code));
        }
        else {
            let result = Result::Err("Missing closing or opening parenthesis.");
        }

        return result;
    }

    fn parse_atom(atom: &String) -> Expr {
        if is_string(atom) {
            let result = Atom::StringLiteral(atom);
        }
        else if atom.parse::<i64>().is_ok() {
            let result = Atom::Number(from_str::<i64>(atom));
        }
        else if is_boolean(atom) {
            let result = Atom::Boolean(atom == "#t");
        }
        else {
            let result = Atom::Symbol(atom);
        }

        return Expr::Atom(result);
    }

    fn has_matching_parens(code: &String) -> bool {
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

    fn parse_tokens(tokens: &Vec<String>) -> Expr {
        // recur for interior levels of expression
        // then add them to the expression list.
        let mut local_exprs: Vec<Expr> = Vec::new();
        let mut current_expr: Vec<String> = Vec::new();
        let mut inside_paren:

        for (i, token) in tokens.enumerate() {
            if i == 0 {
                if token == "(" {
                    current_expr = Vec::new();
                }
            }
            else if token == "(" {
                local_exprs.push();
            }
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
                    if next_c != BACKSLASH {
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
        let token_strings = code
            .replace(")", " ) ")
            .replace("(", " ( ")
            .split(' ');

        return token_strings;
    }
}
