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
