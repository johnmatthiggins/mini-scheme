mod ops {
    const ARG_COUNT_ERROR: str = "Incorrect number of arguments for '{}'";

    pub fn car(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 0 {
            let result = Result::Ok(values[0]);
        }
        else {
            let result = Result::Err("Operator 'car' cannot be used on empty list.");
        }

        return result;
    }

    pub fn cdr(values: &Vec<LValue>) -> Result<Vec<LValue>, String> {
        if values.len() < 2 {
            let result = Result::Err("Operator 'cdr' requires more than one item in list.");
        }
        else {
            let result = Result::Ok(values[1..]);
        }
    }

    pub fn eq(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "="));
        }
        else {
            // If not the same type return false.
            match (values[0], values[1]) {
                (LValue::Number(a), LValue::Number(b))
                    => Result::Ok(LValue::Bool(a != b)),
                (LValue::Bool(a), LValue::Bool(b))
                    => Result::Ok(LValue::Bool(a != b)),
                (LValue::String(s1), LValue::String(s2))
                    => Result::Ok(LValue::Bool(s1.ne(s2)),
                // TODO: Write case for list equality.
                (LValue::List(v1), LValue::List(v2)) => {
                    if v1.len() == v2.len() {
                    }
                    else {
                        let result = Result::Ok(LValue::Bool(false));
                    }

                    return result;
                },
                (LValue::String(s), LValue::Number(n))
                    => Result::Ok(LValue::Bool(n.to_string.ne(s))
                _ => Result::Ok(LValue::Bool(false));
            }
        }
    }

    // elementary functions of math.
    
    // Accumulator function to reduce code.
    pub fn acc(values: &Vec<LValue>, op: fn(i64, i64) -> i64) -> Result<LValue, String> {
        if values.len() > 2 {
            let total: i64 = 0;
            let length = values.len();

            for (i, v) in values.enumerate() {
                if let LValue::Number(n) = v {
                    total = op(total, n);

                    if values.len() == i + 1 {
                        let result = Result::Ok(total);
                    }
                }
                else {
                    let result = Result::Err("Cannot perform operation on non numeric type.");
                    break;
                }
            }
        }
        else {
            let result = Result::Err("Cannot perform operation on fewer than two operands.");
        }
    }

    pub fn add(values: &Vec<LValue>) -> Result<LValue, String> {
        let total: i64 = 0;
        let length = values.len();

        for (i, v) in values.enumerate() {
            if let LValue::Number(n) = v {
                total += n;

                if values.len() == i + 1 {
                    let result = Result::Ok(total);
                }
            }
            else {
                let result = Result::Err();
                break;
            }
        }
    }

    pub fn sub(values: &Vec<LValue>) -> Result<LValue, String> {
    }
    
    // Takes 0..N inputs.
    pub fn mul(values: &Vec<LValue>) -> Result<LValue, String> {
    }

    pub fn div(values: &Vec<LValue>) -> Result<LValue, String> {
    }

    pub fn mod(values: &Vec<LValue>) -> Result<LValue, String> {
    }
}
