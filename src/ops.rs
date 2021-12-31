mod ops {
    const ARG_COUNT_ERROR: str = "Incorrect number of arguments for '{}'";

    // should only take two arguments
    pub fn le(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "le"));
        }
        else {
            // throw type error if not numbers.
            match (values[0], values[1]) {
                (LValue::Number(a), LValue::Number(b)) => Result::Ok(LValue::Bool(a >= b)),
                _ => Result::Err("'le' operator can only be applied to 'Number' values")
            }
        }
    }

    pub fn ne(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "eq"));
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
                // (LValue::List(v1), LValue::List(v2)) => 
                (LValue::String(s), LValue::Number(n))
                    => Result::Ok(LValue::Bool(n.to_string.ne(s)),
                _ => Result::Ok(LValue::Bool(false));
            }
        }
    }

    pub fn lt(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "lt"));
        }
        else {
            // throw type error if not numbers.
            match (values[0], values[1]) {
                (LValue::Number(a), LValue::Number(b)) => Result::Ok(LValue::Bool(a < b)),
                _ => Result::Err("'lt' operator can only be applied to 'Number' values")
        }
    }

    pub fn gt(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "gt"));
        }
        else {
            // throw type error if not numbers.
            match (values[0], values[1]) {
                (LValue::Number(a), LValue::Number(b)) => Result::Ok(LValue::Bool(a > b)),
                _ => Result::Err("'gt' operator can only be applied to 'Number' values")
            }
        }
    }

    pub fn eq(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "eq"));
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
                // (LValue::List(v1), LValue::List(v2)) => 
                (LValue::String(s), LValue::Number(n))
                    => Result::Ok(LValue::Bool(n.to_string.ne(s))
                _ => Result::Ok(LValue::Bool(false));
            }
        }
    }

    pub fn ge(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "ge"));
        }
        else {
            // throw type error if not numbers.
            match (values[0], values[1]) {
                (LValue::Number(a), LValue::Number(b)) => Result::Ok(LValue::Bool(a >= b)),
                _ => Result::Err("'ge' operator can only be applied to 'Number' values")
            }
        }
    }
}
