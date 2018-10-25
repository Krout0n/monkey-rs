use ast::{ASTKind, AST};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(i32),
    Bool(bool),
    Null,
}

// #[derive(Copy)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    fn get(&self, name: String) -> Object {
        if let Some(obj) = self.store.get(&name) {
            obj.clone()
        } else {
            panic!("Undefined variable: {:?}", name);
        }
    }

    fn set(&mut self, name: String, value: Object) -> Object {
        let v = value.clone();
        self.store.insert(name, value);
        v
    }
}

pub struct Evaluator {
    global_env: RefCell<Environment>,
}

impl Evaluator {
    pub fn new() -> Self {
        let global_env = Environment::new();
        Evaluator {
            global_env: RefCell::new(global_env),
        }
    }

    pub fn eval(&self, node: AST) -> Object {
        match node.kind {
            ASTKind::Int(i) => Object::Integer(i),
            ASTKind::Add(lhs, rhs) => match (self.eval(*lhs), self.eval(*rhs)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
                (_, _) => panic!("+ operator supports only integer"),
            },
            ASTKind::Multi(lhs, rhs) => match (self.eval(*lhs), self.eval(*rhs)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Integer(l * r),
                (_, _) => panic!("* operator supports only integer"),
            },
            ASTKind::LT(lhs, rhs) => match (self.eval(*lhs), self.eval(*rhs)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Bool(l < r),
                (_, _) => panic!("< operator supports only integer"),
            },
            ASTKind::LTE(lhs, rhs) => match (self.eval(*lhs), self.eval(*rhs)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Bool(l <= r),
                (_, _) => panic!("<= operator supports only integer"),
            },
            ASTKind::If {
                cond,
                stmt,
                else_stmt,
            } => match self.eval(*cond) {
                Object::Integer(0) | Object::Bool(false) | Object::Null => {
                    if let Some(else_stmt) = else_stmt {
                        self.eval(*else_stmt)
                    } else {
                        Object::Null
                    }
                }
                _ => self.eval(*stmt),
            },
            ASTKind::Bool(b) => Object::Bool(b),
            ASTKind::Return(expr) => self.eval(*expr),
            ASTKind::Compound(stmts) => {
                let mut v = vec![];
                for s in stmts {
                    v.push(self.eval(s))
                }
                v.get(v.len() - 1).unwrap().clone() // todo: fix proper way to return last evaluated obj
            }
            ASTKind::Let { name, expr } => {
                let mut env = self.global_env.borrow_mut();
                env.set(name, self.eval(*expr))
            },
            ASTKind::Ident(s) => {
                self.global_env.borrow().get(s)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Evaluator, Object, AST};
    #[test]
    fn eval_add() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(6),
            ev.eval(AST::add(AST::add(AST::int(1), AST::int(3)), AST::int(2)))
        );
    }

    #[test]
    fn eval_multi() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(100),
            ev.eval(AST::multi(AST::int(20), AST::int(5)))
        )
    }

    #[test]
    fn eval_if() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(0),
            ev.eval(AST::if_stmt(AST::bool(true), AST::int(0), None))
        );
        assert_eq!(
            Object::Null,
            ev.eval(AST::if_stmt(AST::bool(false), AST::int(0), None))
        );
        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::if_stmt(
                AST::bool(false),
                AST::int(0),
                Some(AST::int(2))
            ))
        );
        assert_eq!(
            Object::Integer(0),
            ev.eval(AST::if_stmt(AST::int(1), AST::int(0), Some(AST::int(2))))
        );
        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::if_stmt(
                AST::add(AST::int(1), AST::int(-1)),
                AST::int(0),
                Some(AST::int(2))
            ))
        );
        assert_eq!(
            Object::Integer(20),
            ev.eval(AST::if_stmt(
                AST::bool(true),
                AST::if_stmt(AST::bool(false), AST::int(10), Some(AST::int(20))),
                Some(AST::int(30))
            ))
        )
    }

    #[test]
    fn eval_return() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::return_stmt(AST::add(AST::int(1), AST::int(1))))
        );
    }

    #[test]
    fn eval_compound() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(10),
            ev.eval(AST::compound_statement(vec![AST::int(2), AST::int(10)]))
        )
    }

    #[test]
    fn eval_relational() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Bool(true),
            ev.eval(AST::lt(AST::int(1), AST::int(2)))
        );
    }

    #[test]
    fn eval_let() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::let_stmt("x".to_string(), AST::int(2)))
        );

        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::ident("x".to_string()))
        );
        assert_eq!(
            Object::Integer(3),
            ev.eval(AST::add(AST::ident("x".to_string()), AST::int(1)))
        )
    }
}
