use bigdecimal::BigDecimal;

trait EnvOps {
    fn car(expr: &Expr) -> Result<Expr, String>;

    fn cdr(expr: &Expr) -> Expr;

    fn eq(cdr: &[Expr]) -> Result<Expr, String>;

    fn add(args: &Vec<Expr>) -> Result<Expr, String>;

    fn sub_list(args: &Vec<Expr>);

    fn mul_list(list: &Vec<Expr>);

    fn div_list(list: &Vec<Expr>);

    fn mod_list(list: &Vec<Expr>);
}

pub impl EnvOps for Env {
    // Return first element of list or just empty.
    fn car(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(atom) => Result::Err("'car' can only be applied to lists."),
            Expr::List(list) => values.first() match {
                Some(atom) => Result::Ok(atom),
                None => Result::Err("'car' cannot be applied to empty lists.")
            }
        }
    }

    fn cdr(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Atom(a) => Result::Err("'cdr' can only be applied to lists."),
            Expr::List(list) => Expr::List(list[1..].to_vec())
        }
    }

    fn eq(&mut self, cdr: &[Expr]) -> Result<Expr, String> {
        let first = cdr.first();

        match first {
            Some(s) => {
                let mut is_eq = true;
                
                // Loop through all of them and make sure
                // they are equal to each other.
                for (i, exp) in cdr.iter().enumerate() {
                    is_eq = is_eq && s == exp;
                }
            }
            None => Result::Err("Incorrect argument count for '=' operator."),
        }
    }

    // elementary functions of math.

    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let total: BigDecimal = 0;
        let length = list.len();

        for (i, expr) in list.enumerate() {
            let simple_tree = self.simplify(expr);

            let number = simple_tree match {
                Expr::Atom(atom) => match atom {
                    Atom::Number(n) => Result::Ok(n),
                    _ => Result::Err("Non-number atom cannot have operator '+' applied to it.")
                },
                Expr::List(list) => Result::Err("List cannot have operator '+' applied to it.")
            };

            if number.ok() {
                total = total.add(&number.unwrap());
            }
            else {
                return Result::Err();
            }
        }

        return Result::Ok(Expr::Atom(Atom::Number(total)));
    }

    fn sub_list(&mut self, args: &Vec<Expr>) {
    }

    fn mul_list(&mut self, list: &Vec<Expr>) {
    }

    fn div_list(&mut self, list: &Vec<Expr>) {
    }

    fn mod_list(&mut self, list: &Vec<Expr>) {
    }
}
