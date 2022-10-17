use std::Box;

pub enum LispType {
    Atom(AtomType),
    List(Vec<LispType>)
}

pub enum AtomType {
}

pub struct Scope {
    values: LispType,
}
