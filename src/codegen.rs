use ast::*;

#[allow(dead_code)]
fn gen_code(tree: AST) {
    match tree.kind {
        ASTKind::Int(i) => println!("  pushq %{}", i),
        ASTKind::Add(lhs, rhs) => {
            // gen_code(Box::into_raw(lhs));
            // gen_code(Box::into_raw(lhs));
            gen_code(*lhs);
            gen_code(*rhs);
            println!("  popq %rax");
            println!("  popq %rdx");
            println!("  addq %rax, %rdx");
            println!("  pushq %rax");
        }
        ASTKind::Multi(lhs, rhs) => {
            // gen_code(Box::into_raw(lhs));
            // gen_code(Box::into_raw(lhs));
            gen_code(*lhs);
            gen_code(*rhs);
            println!("  popq %rax");
            println!("  popq %rdx");
            println!("  imul %rax, %rdx");
            println!("  pushq %rax");
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::{gen_code, ASTKind, AST};
    #[test]
    fn simple_tree() {
        // gen_code(AST {
        //     kind: ASTKind::Int(10),
        // });

        // gen_code(AST {
        //     kind: ASTKind::Add(
        //         Box::new(AST {
        //             kind: ASTKind::Int(10),
        //         }),
        //         Box::new(AST {
        //             kind: ASTKind::Int(20),
        //         }),
        //     ),
        // });

        // gen_code(AST {
        //     kind: ASTKind::Add(
        //         Box::new(AST {
        //             kind: ASTKind::Add(
        //                 Box::new(AST {
        //                     kind: ASTKind::Int(1),
        //                 }),
        //                 Box::new(AST {
        //                     kind: ASTKind::Int(2),
        //                 }),
        //             ),
        //         }),
        //         Box::new(AST {
        //             kind: ASTKind::Int(3),
        //         }),
        //     ),
        // });

        // gen_code(AST {
        //     kind: ASTKind::Multi(
        //         Box::new(AST {
        //             kind: ASTKind::Add(
        //                 Box::new(AST {
        //                     kind: ASTKind::Int(1),
        //                 }),
        //                 Box::new(AST {
        //                     kind: ASTKind::Int(2),
        //                 }),
        //             ),
        //         }),
        //         Box::new(AST {
        //             kind: ASTKind::Int(3),
        //         }),
        //     ),
        // });

        gen_code(AST {
            kind: ASTKind::Add(
                Box::new(AST {
                    kind: ASTKind::Add(
                        Box::new(AST {
                            kind: ASTKind::Int(1),
                        }),
                        Box::new(AST {
                            kind: ASTKind::Multi(
                                Box::new(AST {
                                    kind: ASTKind::Int(2),
                                }),
                                Box::new(AST {
                                    kind: ASTKind::Int(3),
                                }),
                            ),
                        }),
                    ),
                }),
                Box::new(AST {
                    kind: ASTKind::Int(4),
                }),
            ),
        });
    }
}
