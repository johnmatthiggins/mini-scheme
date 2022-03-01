// Types of operations on expressions
// * eval_list: Evalutes list of expressions.
// * apply: Applies operation to an expression
// * eval_car_cdr: Evaluates the first part of the last as the operator
//      and all the other elements as operands.
// * simplify: Returns if it's an atom, recursively calls
//      eval_list if it's a list of expressions.
// * get_symbol: Attempts to simplify argument, then returns what it is mapped too
//               if it simplifies to an atom.
// * execute_lambda:
