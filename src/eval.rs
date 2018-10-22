use ast::{ASTKind, AST};

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i32),
    Bool(bool),
}

pub fn eval(node: AST) -> Object {
    match node.kind {
        ASTKind::Int(i) => Object::Integer(i),
        ASTKind::Add(lhs, rhs) => match (eval(*lhs), eval(*rhs)) {
            (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
            (_, _) => panic!("+ operand supports only integer"),
        },
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
}
