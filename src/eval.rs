use ast::{ASTKind, AST};

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(i32),
    Bool(bool),
    Null,
}

pub fn eval(node: AST) -> Object {
    match node.kind {
        ASTKind::Int(i) => Object::Integer(i),
        ASTKind::Add(lhs, rhs) => match (eval(*lhs), eval(*rhs)) {
            (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
            (_, _) => panic!("+ operator supports only integer"),
        },
        ASTKind::Multi(lhs, rhs) => match (eval(*lhs), eval(*rhs)) {
            (Object::Integer(l), Object::Integer(r)) => Object::Integer(l * r),
            (_, _) => panic!("* operator supports only integer"),
        },
        ASTKind::If {
            cond,
            stmt,
            else_stmt,
        } => match eval(*cond) {
            Object::Integer(0) | Object::Bool(false) | Object::Null => {
                if let Some(else_stmt) = else_stmt {
                    eval(*else_stmt)
                } else {
                    Object::Null
                }
            }
            _ => eval(*stmt),
        },
        ASTKind::Bool(b) => Object::Bool(b),
        ASTKind::Return(expr) => eval(*expr),
        ASTKind::Compound(stmts) => {
            let mut v = vec![];
            for s in stmts {
                v.push(eval(s))
            }
            v.get(v.len() - 1).unwrap().clone() // todo: fix proper way to return last evaluated obj
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{eval, Object, AST};
    #[test]
    fn eval_integer() {
        let one = AST::int(1);
        assert_eq!(Object::Integer(1), eval(one));
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Object::Integer(6),
            eval(AST::add(AST::add(AST::int(1), AST::int(3)), AST::int(2)))
        );
    }

    #[test]
    fn eval_multi() {
        assert_eq!(
            Object::Integer(100),
            eval(AST::multi(AST::int(20), AST::int(5)))
        )
    }

    #[test]
    fn eval_if() {
        assert_eq!(
            Object::Integer(0),
            eval(AST::if_stmt(AST::bool(true), AST::int(0), None))
        );
        assert_eq!(
            Object::Null,
            eval(AST::if_stmt(AST::bool(false), AST::int(0), None))
        );
        assert_eq!(
            Object::Integer(2),
            eval(AST::if_stmt(
                AST::bool(false),
                AST::int(0),
                Some(AST::int(2))
            ))
        );
        assert_eq!(
            Object::Integer(0),
            eval(AST::if_stmt(AST::int(1), AST::int(0), Some(AST::int(2))))
        );
        assert_eq!(
            Object::Integer(2),
            eval(AST::if_stmt(
                AST::add(AST::int(1), AST::int(-1)),
                AST::int(0),
                Some(AST::int(2))
            ))
        );
        assert_eq!(
            Object::Integer(20),
            eval(AST::if_stmt(
                AST::bool(true),
                AST::if_stmt(AST::bool(false), AST::int(10), Some(AST::int(20))),
                Some(AST::int(30))
            ))
        )
    }

    #[test]
    fn eval_return() {
        assert_eq!(
            Object::Integer(2),
            eval(AST::return_stmt(AST::add(AST::int(1), AST::int(1))))
        );
    }

    #[test]
    fn eval_compound() {
        assert_eq!(
            Object::Integer(10),
            eval(AST::compound_statement(vec![AST::int(2), AST::int(10)]))
        )
    }
}
