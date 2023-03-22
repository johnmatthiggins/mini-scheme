mod boolean;
mod built_in;
mod env;
mod lex;
mod math;
mod syntax;
mod sys;
mod test;

use crate::env::{Env, Eval};
use crate::syntax::*;
use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};
use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::{Write, BufRead};
use std::fs::File;
use std::fs;
use std::path::Path;
use std::env::args;
use std::mem;
use std::rc;
use bigdecimal::BigDecimal;
use rustyline::DefaultEditor;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "~> ";

fn main() {
    let args: Vec<String> = args().collect();

    if let Some(arg) = args.get(1) {
        if arg.as_str() == "--repl"  || arg.as_str() == "-r" {
            repl_mode();
        } else {
            interpreter_mode(&arg);
        }
    } else {
        repl_mode();
    }
}

fn repl_mode() {
    print!("Mini-Scheme Version {}\n", VERSION);

    // Holds all the predefined functions and values for REPL session.
    let mut env: Env = Box::new(HashMap::new());
    let mut failed = false;
    let mut line_reader = rustyline::DefaultEditor::new().expect("REASON");

    #[cfg(feature = "with-file-history")]
    if line_reader.load_history(".lisp-history").is_err() {
        println!("No previous history.");
    }

    loop {
        let input = line_reader.readline(PROMPT).expect("User input could not be read...");

        if input.len() > 0 {
            match input.as_str() {
                "exit" => {
                    print!("\n");
                    break;
                },
                _ => {
                    line_reader.add_history_entry(input.as_str());
                    let first_char = input.chars().next().unwrap();

                    if first_char != ';' {
                        let result = env.eval(&input);

                        match result {
                            Ok(expr) => {
                                failed = false;
                                println!("{}", print_tree(&expr, &true));
                            }
                            Err(msg) => {
                                failed = true;
                                println!("{}", &msg);
                            }
                        };
                    }
                }
            }
        }
    }
}

fn interpret_line(input: &String, env: &mut Env) {
    if input.len() > 0 {
        let first_char = input.chars().next().unwrap();

        if first_char != ';' {
            let result = env.eval(input);

            // match result {
            //     Ok(expr) => {
            //         failed = false;
            //         println!("{}", print_tree(&expr));
            //     }
            //     Err(msg) => {
            //         failed = true;
            //         println!("{}", &msg);
            //     }
            // };
        }
    }
}

fn interpreter_mode(path: &String) {
    let text = fs::read_to_string(path.as_str())
        .unwrap()
        .parse()
        .unwrap();

    interpret_raw_text(&text);
}

fn interpret_raw_text(text: &String) {
    let mut env: Env = Box::new(HashMap::new());

    let lines = chunk_file(&text);

    for line in lines {
        interpret_line(&line, &mut env);
    }
}

fn trim_newline(s: &mut String) -> String {
    let mut trimmed = s.clone();

    if trimmed.ends_with('\n') {
        trimmed.pop();
        if trimmed.ends_with('\r') {
            trimmed.pop();
        }
    }

    trimmed
}

fn chunk_file(text: &String) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut chunk: String = String::new();
    let mut nesting_level: u32 = 0;

    for c in text.chars() {
        if nesting_level > 0 || c == '(' {
            if c == '\n' {
                chunk.push(' ');
            } else {
                chunk.push(c);
            }
        } else if chunk.len() != 0 {
            chunks.push(chunk.clone());
            chunk.clear();
        }

        if c == '(' {
            nesting_level += 1;
        } else if c == ')' {
            nesting_level -= 1;
        }
    }

    // dbg!(&chunks);

    chunks
}
