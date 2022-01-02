pub mod syntax {
   enum Atom {
       Boolean(bool),
       StringLiteral(String),
       Number(i64),
       Symbol(String)
   }

   enum Expr {
        List(Vec<Expr>),
        Atom(Atom)
   }
}
