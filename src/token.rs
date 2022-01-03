/**
 * @author johnmatthiggins@gmail.com
 */

mod syntax;

use std::vec::Vec;
use crate::syntax::Expr;
use crate::syntax::Atom;

// const DEFINE_KW: str = "define";
// const LAMBDA_KW: str = "lambda";
// const QUOTE_KW: str = "quote";
// const ADD_KW: str = "add";
// const SUB_KW: str = "sub";
// const MUL_KW: str = "mul";
// const DIV_KW: str = "div";
// const MOD_KW: str = "mod";

// pub fn to_string(lVal: LValue) -> String {
//     match lVal {
//         LValue::Number(n) => n.to_string(),
//         LValue::Bool(b) => b.to_string(),
//         LValue::String(s) => format!("\"{}\"", s),
//         LValue::List(v) => {
//             let mut text: String = "(";

//             for (i, val) in v.enumerate() {
//                 // Lol recursion
//                 if i == 0 {
//                     text.push_str(format!("{}", to_string(val)));
//                 }
//                 else {
//                     text.push_str(format!(", {}", to_string(val)));
//                 }
//             }

//             text.push_str(")"); 

//             return text;
//         }
//     }
// }
