use std::collections::HashMap;

use crate::{
    ast::{Assign, BinExp, Expr, Ident, Literal, Program, Statement},
    object::Object,
};

#[derive(Default)]
pub struct Eval {
    env: HashMap<String, Object>,
}

impl Eval {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn eval(&mut self, program: Program) -> Vec<Object> {
        let mut result = vec![];

        for statement in program {
            result.push(self.eval_stmt(statement));
        }

        result
    }

    fn eval_stmt(&mut self, statement: Statement) -> Object {
        match statement {
            Statement::Assign(Assign { target, value }) => {
                let value = self.eval_expr(value);
                *self.env.entry(target.0).or_insert(Object::Void) = value.clone();

                value
            }
            Statement::Expr(expr) => self.eval_expr(expr),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::Ident(Ident(name)) => self
                .env
                .get(&name)
                .unwrap_or_else(|| panic!("Undefined var {}", name))
                .clone(),
            Expr::BinExp(BinExp { op, left, right }) => {
                let l = self.eval_expr(*left);
                let r = self.eval_expr(*right);

                match op.as_str() {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => l / r,
                    "%" => l % r,
                    "^" => {
                        let r_int = match r {
                            Object::Void | Object::String(_) => panic!(),
                            Object::Int(v) => v,
                            Object::Float(v) => v as i64,
                        };

                        let mut result = Object::Int(1);
                        for _ in 0..r_int {
                            result = result * l.clone();
                        }

                        result
                    }
                    _ => {
                        panic!("Unexpected operator: {}", op);
                    }
                }
            }
            Expr::Literal(lit) => match lit {
                Literal::Int(v) => Object::Int(v),
                Literal::Float(v) => Object::Float(v),
                Literal::String(v) => Object::String(v),
            },
        }
    }
}
