pub mod syntax {
   enum Atom {
       Boolean(bool),
       StringLiteral(String),
       Number(i64),
       Symbol(String)
   }

   enum Expr {
        Vec<Expr>,
        Atom(Atom)
   }
}
