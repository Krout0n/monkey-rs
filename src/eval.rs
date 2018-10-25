use ast::{ASTKind, AST};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(i32),
    Bool(bool),
    FnDef {
        args: Vec<String>,
        stmts: Vec<AST>,
        env: RefCell<Environment>,
    },
    Null,
}

impl Object {
    fn func(args: Vec<String>, stmts: Vec<AST>) -> Self {
        Object::FnDef {
            args,
            stmts,
            env: RefCell::new(Environment::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    pub global_env: RefCell<Environment>,
}

impl Evaluator {
    pub fn new() -> Self {
        let global_env = Environment::new();
        Evaluator {
            global_env: RefCell::new(global_env),
        }
    }

    pub fn eval(&self, node: AST, env: &RefCell<Environment>) -> Object {
        match node.kind {
            ASTKind::Int(i) => Object::Integer(i),
            ASTKind::Add(lhs, rhs) => match (self.eval(*lhs, env), self.eval(*rhs, env)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
                (_, _) => panic!("+ operator supports only integer"),
            },
            ASTKind::Minus(lhs, rhs) => match (self.eval(*lhs, env), self.eval(*rhs, env)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
                (_, _) => panic!("+ operator supports only integer"),
            },
            ASTKind::Multi(lhs, rhs) => match (self.eval(*lhs, env), self.eval(*rhs, env)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Integer(l * r),
                (_, _) => panic!("* operator supports only integer"),
            },
            ASTKind::LT(lhs, rhs) => match (self.eval(*lhs, env), self.eval(*rhs, env)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Bool(l < r),
                (_, _) => panic!("< operator supports only integer"),
            },
            ASTKind::LTE(lhs, rhs) => match (self.eval(*lhs, env), self.eval(*rhs, env)) {
                (Object::Integer(l), Object::Integer(r)) => Object::Bool(l <= r),
                (_, _) => panic!("<= operator supports only integer"),
            },
            ASTKind::If {
                cond,
                stmt,
                else_stmt,
            } => match self.eval(*cond, env) {
                Object::Integer(0) | Object::Bool(false) | Object::Null => {
                    if let Some(else_stmt) = else_stmt {
                        self.eval(*else_stmt, env)
                    } else {
                        Object::Null
                    }
                }
                _ => self.eval(*stmt, env),
            },
            ASTKind::Bool(b) => Object::Bool(b),
            ASTKind::Return(expr) => self.eval(*expr, env),
            ASTKind::Compound(stmts) => {
                let mut v = vec![];
                for s in stmts {
                    v.push(self.eval(s, env))
                }
                v.get(v.len() - 1).unwrap().clone() // todo: fix proper way to return last evaluated obj
            }
            ASTKind::Let { name, expr } => {
                let value = self.eval(*expr, env);
                let mut env = env.borrow_mut();
                env.set(name, value)
            }
            ASTKind::Ident(s) => env.borrow().get(s),
            ASTKind::FnDef { args, stmts } => Object::func(args, stmts),
            ASTKind::FnCall { name, args: exprs } => {
                let values: Vec<Object> = exprs.into_iter().map(|x| self.eval(x, &env)).collect();
                let fnobj = self.eval(AST::ident(name.clone()), env);
                if let Object::FnDef { args, stmts, env } = fnobj.clone() {
                    env.borrow_mut().set(name, fnobj);
                    for (name, value) in args.iter().zip(values.iter()) {
                        env.borrow_mut().set(name.clone(), value.clone());
                    }
                    let mut v = vec![];
                    for s in stmts {
                        v.push(self.eval(s, &env))
                    }
                    v.get(v.len() - 1).unwrap().clone() // todo: fix proper way to return last evaluated obj
                } else {
                    panic!("you tried to call undefined function.");
                }
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
            ev.eval(
                AST::add(AST::add(AST::int(1), AST::int(3)), AST::int(2)),
                &ev.global_env
            )
        );
    }

    #[test]
    fn eval_multi() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(100),
            ev.eval(AST::multi(AST::int(20), AST::int(5)), &ev.global_env)
        )
    }

    #[test]
    fn eval_if() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(0),
            ev.eval(
                AST::if_stmt(AST::bool(true), AST::int(0), None),
                &ev.global_env
            )
        );
        assert_eq!(
            Object::Null,
            ev.eval(
                AST::if_stmt(AST::bool(false), AST::int(0), None),
                &ev.global_env
            )
        );
        assert_eq!(
            Object::Integer(2),
            ev.eval(
                AST::if_stmt(AST::bool(false), AST::int(0), Some(AST::int(2))),
                &ev.global_env
            )
        );
        assert_eq!(
            Object::Integer(0),
            ev.eval(
                AST::if_stmt(AST::int(1), AST::int(0), Some(AST::int(2))),
                &ev.global_env
            )
        );
        assert_eq!(
            Object::Integer(2),
            ev.eval(
                AST::if_stmt(
                    AST::add(AST::int(1), AST::int(-1)),
                    AST::int(0),
                    Some(AST::int(2))
                ),
                &ev.global_env
            )
        );
        assert_eq!(
            Object::Integer(20),
            ev.eval(
                AST::if_stmt(
                    AST::bool(true),
                    AST::if_stmt(AST::bool(false), AST::int(10), Some(AST::int(20))),
                    Some(AST::int(30))
                ),
                &ev.global_env
            )
        )
    }

    #[test]
    fn eval_return() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(2),
            ev.eval(
                AST::return_stmt(AST::add(AST::int(1), AST::int(1))),
                &ev.global_env
            )
        );
    }

    #[test]
    fn eval_compound() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(10),
            ev.eval(
                AST::compound_statement(vec![AST::int(2), AST::int(10)]),
                &ev.global_env
            )
        )
    }

    #[test]
    fn eval_relational() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Bool(true),
            ev.eval(AST::lt(AST::int(1), AST::int(2)), &ev.global_env)
        );
    }

    #[test]
    fn eval_let() {
        let ev = Evaluator::new();
        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::let_stmt("x".to_string(), AST::int(2)), &ev.global_env)
        );

        assert_eq!(
            Object::Integer(2),
            ev.eval(AST::ident("x".to_string()), &ev.global_env)
        );
        assert_eq!(
            Object::Integer(3),
            ev.eval(
                AST::add(AST::ident("x".to_string()), AST::int(1)),
                &ev.global_env
            )
        )
    }

    #[test]
    fn eval_func() {
        let ev = Evaluator::new();
        ev.eval(
            AST::let_stmt(
                "x".to_string(),
                AST::fn_def(vec![], vec![AST::return_stmt(AST::int(1))]),
            ),
            &ev.global_env,
        );
        assert_eq!(
            Object::Integer(1),
            ev.eval(AST::fn_call("x".to_string(), vec![]), &ev.global_env)
        );

        // let x = fn(x) {  return x + 1;}
        ev.eval(
            AST::let_stmt(
                "x".to_string(),
                AST::fn_def(
                    vec!["x".to_string()],
                    vec![AST::return_stmt(AST::add(
                        AST::int(1),
                        AST::ident("x".to_string()),
                    ))],
                ),
            ),
            &ev.global_env,
        );

        assert_eq!(
            Object::Integer(2),
            ev.eval(
                AST::fn_call("x".to_string(), vec![AST::int(1)]),
                &ev.global_env
            )
        );
    }

    // let twice = fn(f, x) {
    //   return f(f(x));
    // };
    // twice(fn(a) { return a + 1;} 0);
    #[test]
    fn eval_closure() {
        let ev = Evaluator::new();
        ev.eval(
            AST::let_stmt(
                "twice".to_string(),
                AST::fn_def(
                    vec!["f".to_string(), "x".to_string()],
                    vec![AST::fn_call(
                        "f".to_string(),
                        vec![AST::fn_call(
                            "f".to_string(),
                            vec![AST::ident("x".to_string())],
                        )],
                    )],
                ),
            ),
            &ev.global_env,
        );
        assert_eq!(
            Object::Integer(2),
            ev.eval(
                AST::fn_call(
                    "twice".to_string(),
                    vec![AST::fn_def(
                        vec!["a".to_string()],
                        vec![AST::return_stmt(AST::add(
                            AST::ident("a".to_string()),
                            AST::int(1)
                        ))]
                    ), AST::int(0)]
                ),
                &ev.global_env
            )
        );
    }
}
