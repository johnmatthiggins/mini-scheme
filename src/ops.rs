mod ops {
    const ARG_COUNT_ERROR: str = "Incorrect number of arguments for '{}'";

    // should only take two arguments
    pub fn eq(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "eq"));
        }
        else {
        }
    }

    pub fn ne(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "ne"));
        }
        else {
        }
    }

    pub fn lt(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "lt"));
        }
        else {
        }
    }

    pub fn gt(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "gt"));
        }
        else {
        }
    }

    pub fn le(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "le"));
        }
        else {
            // If not the same type return false.
            match (values[0], values[1]) {
                (LValue::Number(a), LValue::Number(b)) => a == b,
                (LValue::Bool(a), LValue::Bool(b)) => a == b,
                (LValue::String(s1), LValue::String(s2)) => s1.eq(s2),
                // TODO: Write case for list equality.
                // (LValue::List(v1), LValue::List(v2)) => 
                _ => false
            }
        }
    }

    pub fn ge(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "ge"));
        }
        else {
        }
    }
}
