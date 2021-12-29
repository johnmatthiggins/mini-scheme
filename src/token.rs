/**
 * @author johnmatthiggins@gmail.com
 */

use std::vec::Vec;

mod token {
    const DEFINE_KW: str = "define";
    const LAMBDA_KW: str = "lambda";
    const QUOTE_KW: str = "quote";
    const ADD_KW: str = "add";
    const SUB_KW: str = "sub";
    const MUL_KW: str = "mul";
    const DIV_KW: str = "div";
    const MOD_KW: str = "mod";

    enum LValue {
        Number(i64),
        Bool(bool)
        String(String),
        List(Vec<LValue>),
    }

    enum Token {
        Value(Value)
        Data(DataToken)
    }
}
