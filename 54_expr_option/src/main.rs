#![feature(let_chains)]

use anyhow::{bail, Context, Result};

#[derive(Debug, Clone)]
enum Expr {
    Value(i32),
    Div(Box<(Expr, Expr)>),
}
use crate::Expr::{Div, Value};

fn eval(e: &Expr) -> Option<i32> {
    match e {
        Value(v) => Some(*v),
        Div(operands) => checked_div(eval(&operands.0)?, eval(&operands.1)?),
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

// return Some in case we can compute division
fn checked_div_log(m: i32, n: i32) -> (Option<i32>, String) {
    if n == 0 {
        (None, format!("error: Div-by-zero(m={m}/n={n})"))
    } else {
        let r = m / n;
        (Some(r), format!("Div(m={m}/n={n})={r}"))
    }
}

fn eval_log(e: &Expr) -> (Option<i32>, String) {
    match e {
        Value(v) => (Some(*v), format!("Value({v})")),
        Div(operands) => {
            let ((vl, ll), (vr, lr)) = (eval_log(&operands.0), eval_log(&operands.1));
            if let Some(v1) = vl
                && let Some(v2) = vr
            {
                let (v, l) = checked_div_log(v1, v2);
                (v, format!("{l} where (m={ll}, n={lr})"))
            } else {
                (None, format!("error: Div-wrong-argument({ll},{lr}"))
            }
        }
    }
}

// return Some in case we can compute division
fn div_result(m: i32, n: i32) -> Result<i32> {
    if n == 0 {
        bail!("Division by zero: Div({m}/{n}")
    } else {
        Ok(m / n)
    }
}

fn eval_result(e: &Expr) -> Result<i32> {
    match e {
        Value(v) => Ok(*v),
        Div(operands) => div_result(
            eval_result(&operands.0)
                .with_context(|| format!("Failed to evaluate LH expression {operands:?}"))?,
            eval_result(&operands.1)
                .with_context(|| format!("Failed to evaluate RH expression {operands:?}"))?,
        ),
    }
}

fn main() {
    let e = Div(Box::new((Value(20), Value(5))));
    println!("- e  = {e:?} -> eval(e)  = {:?}\n", eval(&e));
    println!("- e  = {e:?} -> eval_log(e)  = {:?}\n", eval_log(&e));

    let e2 = Div(Box::new((Value(20), Value(0))));
    println!("- e2 = {e2:?} -> eval(e)  = {:?}", eval(&e2));
    println!("- e2 = {e2:?} -> eval_log(e)  = {:?}", eval_log(&e2));

    let e3 = Div(Box::new((
        Div(Box::new((Value(20), Value(0)))),
        Div(Box::new((Value(20), Value(5)))),
    )));
    println!("- e3 = {e3:?} -> eval_log(e)  = {:?}", eval_log(&e3));
    println!("- e3 = {e3:?} -> eval_result(e)  = {:?}", eval_result(&e3));

    let e4 = Div(Box::new((
        Div(Box::new((Value(20), Value(2)))),
        Div(Box::new((Value(20), Value(5)))),
    )));
    println!("- e4 = {e4:?} -> eval_log(e)  = {:?}", eval_log(&e4));
    println!("- e4 = {e4:?} -> eval_result(e)  = {:?}", eval_result(&e4));

    let e5 = Div(Box::new((
        Div(Box::new((Value(20), Value(2)))),
        Div(Box::new((
            Div(Box::new((Value(20), Value(2)))),
            Div(Box::new((Value(20), Value(0)))),
        ))),
    )));
    println!("- e5 = {e5:?} -> eval_result(e)  = {:?}", eval_result(&e5));
}
