use crate::syntax::Expr;
use crate::syntax::Atom;

// Operator info for applying to lists in apply_op() and apply_neg_op().
struct OpInfo {
    name: String,
    op_fn: fn(&BigDecimal, &BigDecimal) -> BigDecimal,
    default: BigDecimal
}

fn eq_op(args: &Vec<Expr>) -> Result<Expr, String> {
    let first = args.first()
        .ok_or("Operator '=' must have at least one argument.".to_string());
    
    match first {
        Ok(s) => {
            let mut is_eq = true;

            // Loop through all of the args and make sure they are
            // equal to each other.
            for expr in args[1..].iter() {
                match expr {
                    Ok(v) => {
                        is_eq = is_eq && s == v;
                    },
                    Err(msg) => {
                        return Err(msg);
                    }
                }
            }

            Result::Ok(Expr::Atom(Atom::Boolean(is_eq)));
        }
        Err(msg) => Result::Err(msg),
    }
}

fn add_op(args: &Vec<Expr>) -> Result<Expr, String> {
    let op_info = OpInfo {
        name: String::from("+"),
        op_fn: |a, b| a + b,
        default: BigDecimal::from(0)
    };

    apply_op(default, args, op_info.op_fn, op_info.name);
}

fn sub_op(args: &Vec<Expr>) -> Result<Expr, String> {
    let op_info = OpInfo {
        name: String::from("-"),
        op_fn: |a, b| a - b,
        default: BigDecimal::from(0)
    };

    apply_neg_op(args, &op_info);
}

fn mul_op(args: &Vec<Expr>) -> Result<Expr, String> {
    let op_info = OpInfo {
        name: String::from("*"),
        op_fn: |a, b| a * b,
        default: BigDecimal::from(1)
    };

    apply_op(default, args, op_info.op_fn, op_info.name);
}

fn div_op(args: &Vec<Expr>) -> Result<Expr, String> {
    let op_info = OpInfo {
        name: String::from("/"),
        op_fn: |a, b| a / b,
        default: BigDecimal::from(1)
    };

    apply_neg_op(args, &op_info);
}

fn mod_op(args: &Vec<Expr>) -> Result<Expr, String> {
    if args.len() == 2 {
        // Specifies default but it'll never actually be used.
        let op_info = OpInfo {
            name: String::from("%"),
            op_fn: |a, b| a.rem(b),
            default: BigDecimal::from(69)
        };

        apply_neg_op(args, &op_info);
    }
    else {
        Err("Incorrect argument count for '%' operator.".to_string());
    }
}

// Every expression must have already been converted to number.
fn apply_neg_op(args: &Vec<Expr>, info: &OpInfo) -> Result<Expr, String> {
    let total = args.first()
        .ok_or(format!("Incorrect argument count for '{}' operator.", info.name)
               .to_string())
        .and_then(|x| match x {
            Expr::Atom(atom) => match atom {
                Atom::Number(n) => Ok(n.to_owned()),
                _ => Err(format!("Cannot perform '{}' operator on non-numeric type.", info.name)
                         .to_string())
            },
            Expr::List(_) =>
                Err(format!("Cannot perform '{}' operator on non-numeric type.", info.name)
                    .to_string())
        });

    if args.len() > 1 {
        total.and_then(|n| apply_op(n, &args[1..].to_vec(), info.op_fn, &info.name));
    }
    else {
        total.map(|x| Ok(Expr::Atom(Atom::Number((info.op_fn)(&info.default, &x)))))
            .unwrap_or(
                Err(format!("Cannot perform '{}' operator on non-numeric type.", info.name)
                    .to_string()));
    }
}

// Every expression must have already been converted to a number.
fn apply_op(car: BigDecimal, cdr: &Vec<Expr>, op: fn(&BigDecimal, &BigDecimal) -> BigDecimal,
            op_name: &str) -> Result<Expr, String> {
    let mut total = car;

    for expr in cdr.into_iter() {
        match expr {
            Ok(tree) => {
                let number = match tree {
                    Expr::Atom(atom) => match atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            format!("Non-number atom cannot have operator '{}' applied to it.", op_name)
                                .to_string())
                    },
                    Expr::List(_) => Result::Err(
                        format!("List cannot have operator '{}' applied to it.", op_name).to_string())
                };

                match number {
                    Ok(n) => {
                        total = op(&total, &n);
                    },
                    Err(msg) => {
                        return Result::Err(msg);
                    }
                }
            },
            Err(msg) => return Result::Err(msg)
        };
    }

    Result::Ok(Expr::Atom(Atom::Number(total)));
}
