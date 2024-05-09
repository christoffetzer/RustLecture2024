#[derive(Debug, Clone)]
enum Expr {
    Value(i32),
    Div(Box<(Expr, Expr)>),
}
use crate::Expr::{Div, Value};

fn eval(e: &Expr) -> i32 {
    match e {
        Value(v) => *v,
        Div(operands) => eval(&operands.0) / eval(&operands.1),
    }
}

fn main() {
    let e = Expr::Div(Box::new((Expr::Value(20), Expr::Value(5))));

    let e2 = Div(Box::new((Value(20), Value(5))));
    println!("- e  = {e:?} -> eval(e)  = {}\n", eval(&e));
    println!("- e2 = {e2:?} -> eval(e)  = {}", eval(&e2));
}
