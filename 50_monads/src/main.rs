use num::CheckedAdd;
// std::ops::Div; see https://doc.rust-lang.org/std/ops/trait.Div.html
// std::cmp::Eq;  see https://doc.rust-lang.org/std/cmp/trait.Eq.html

// Recursive Data Structure

/*
#[derive(Debug, Clone)]
enum Expr {
    Value(i32),
    Div(Expr, Expr),
}
// recursive type `Expr` has infinite size
*/

// needs some indirection since size not known compile time
#[derive(Debug, Clone)]
enum Expr {
    Value(i32),
    Div(Box<(Expr, Expr)>),
    Or(Box<(Expr, Expr)>),
}

// enable use of Value, Div, Or instead of Expr::Value, Expr::Div, Expr::Or
use crate::Expr::{Div, Or, Value};

fn eval_1(e: Expr) -> Option<i32> {
    match e {
        Value(v) => Some(v),
        Div(operands) => checked_div(eval(operands.0)?, eval(operands.1)?),
        Or(operands) => safe_or(eval(operands.0), eval(operands.1)),
    }
}

// needs some indirection since size not known compile time
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum GenericExpr<T> {
    Value(T),
    Div(Box<(GenericExpr<T>, GenericExpr<T>)>),
    Add(Box<(GenericExpr<T>, GenericExpr<T>)>),
    Or(Box<(GenericExpr<T>, GenericExpr<T>)>), // value of left hand value if not None, otherwise right hand value
    And(Box<(GenericExpr<T>, GenericExpr<T>)>), // value of right hand value but only if both values are defined
}

fn generic_eval<T: std::ops::Div<Output = T> + std::cmp::Eq + num::traits::Zero + CheckedAdd>(
    e: GenericExpr<T>,
) -> Option<T> {
    match e {
        GenericExpr::Value::<T>(v) => Some(v),
        GenericExpr::Div::<T>(operands) => generic_safe_div::<T>(
            generic_eval::<T>(operands.0)?,
            generic_eval::<T>(operands.1)?,
        ),
        // ensure that we only evaluate right hand if the left hand is None
        GenericExpr::Or::<T>(operands) => {
            if let Some(v) = generic_eval::<T>(operands.0) {
                Some(v)
            } else {
                generic_eval::<T>(operands.1)
            }
        }
        // ensure that we only evaluate right hand if the left hand is None
        GenericExpr::And::<T>(operands) => {
            generic_eval::<T>(operands.0)?;
            generic_eval::<T>(operands.1)
        }
        GenericExpr::Add::<T>(operands) => {
            generic_eval::<T>(operands.0)?.checked_add(&generic_eval::<T>(operands.1)?)
        }
    }
}

// return Some in case we can compute division
fn generic_safe_div<T: std::ops::Div<Output = T> + std::cmp::Eq + num::traits::Zero>(
    m: T,
    n: T,
) -> Option<T> {
    if n == T::zero() {
        None
    } else {
        Some(m.div(n))
    }
}

fn eval(e: Expr) -> Option<i32> {
    match e {
        Value(v) => Some(v),
        Div(operands) => safe_div(eval(operands.0), eval(operands.1)),
        Or(operands) => safe_or(eval(operands.0), eval(operands.1)),
    }
}

// return Some in case we can compute division
fn safe_div(m: Option<i32>, n: Option<i32>) -> Option<i32> {
    if n? == 0 {
        None
    } else {
        Some(m? / n?)
    }
}

// return Some in case we can compute division
fn checked_div(m: i32, n: i32) -> Option<i32> {
    if n == 0 {
        None
    } else {
        Some(m / n)
    }
}

fn safe_or(m: Option<i32>, n: Option<i32>) -> Option<i32> {
    match (m, n) {
        (Some(_), _) => m,
        (_, _) => n,
    }
}

fn main() {
    // wrap: 20 -> Some(20)
    // wrap: 4 -> Some(4)
    println!("Expression = {v:?} ", v = safe_div(Some(10), Some(0)));
    println!("Expression = {v:?} ", v = safe_div(Some(20), Some(4)));

    println!(
        "Expression = {v:?} ",
        v = safe_or(safe_div(Some(10), Some(0)), safe_div(Some(20), Some(4)))
    );

    let e = Or(Box::new((Div(Box::new((Value(200), Value(0)))), Value(10))));
    println!("Expression = {v:?} ", v = eval_1(e));

    let e_g = GenericExpr::Add::<u64>(Box::new((
        GenericExpr::Div::<u64>(Box::new((
            GenericExpr::Value(200u64),
            GenericExpr::Value(10u64),
        ))),
        GenericExpr::Value(10),
    )));
    println!("Expression = {v:?} ", v = generic_eval(e_g));
}
