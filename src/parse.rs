use std::vec::Vec;
use std::result::Result;
use std::collections::LinkedList;

mod parse {
    enum Tree {
        Parent(Vec<Tree>, String),
        Leaf(String),
    }

    pub fn parse(code: &String) -> Result<Vec<Token>, String> {
        let is_matching = has_matching_parens(code);

        if is_matching {
            let text_tokens = split_with_parens(code);
            let string_tree = 
        }
        else {
            let result = Err("Missing closing or opening parenthesis.");
        }
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

    fn to_trees(tokens: &Vec<String>) -> Vec<Tree> {
        let mut trees: Vec<Tree> = Vec::new();
        let mut index = 0;
    }
}
