use mako_core::swc_ecma_ast::*;
use mako_core::swc_ecma_utils::private_ident;
use mako_core::swc_ecma_visit::VisitMut;
use swc_core::common::DUMMY_SP;

const DEFAULT_COMPONENT_NAME: &str = "Component$$";

pub struct NamedExportDefault {}

impl NamedExportDefault {
    pub fn new() -> Self {
        NamedExportDefault {}
    }
}

impl VisitMut for NamedExportDefault {
    fn visit_mut_module_item(&mut self, item: &mut ModuleItem) {
        // modify in module_item so hygiene_with_config rename will work
        if let ModuleItem::ModuleDecl(module_decl) = item {
            match module_decl {
                ModuleDecl::ExportDefaultExpr(decl) => {
                    if let Expr::Arrow(arrow_expr) = decl.expr.as_ref() {
                        if arrow_expr.is_async || arrow_expr.is_generator {
                            return;
                        }
                        let ArrowExpr {
                            params,
                            body,
                            is_async,
                            is_generator,
                            return_type,
                            type_params,
                            span,
                            ..
                        } = arrow_expr.clone();
                        *item = ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(
                            ExportDefaultDecl {
                                span: DUMMY_SP,
                                decl: DefaultDecl::Fn(FnExpr {
                                    ident: Some(private_ident!(DEFAULT_COMPONENT_NAME)),
                                    function: Box::new(Function {
                                        params: params
                                            .iter()
                                            .cloned()
                                            .map(|pat: Pat| Param {
                                                span,
                                                decorators: vec![],
                                                pat,
                                            })
                                            .collect::<Vec<_>>(),
                                        body: Some(match *body {
                                            BlockStmtOrExpr::BlockStmt(block_stmt) => block_stmt,
                                            BlockStmtOrExpr::Expr(expr) => BlockStmt {
                                                span,
                                                stmts: vec![Stmt::Return(ReturnStmt {
                                                    span,
                                                    arg: Some(expr),
                                                })],
                                            },
                                        }),
                                        is_async,
                                        is_generator,
                                        span,
                                        return_type,
                                        type_params,
                                        decorators: vec![],
                                    }),
                                }),
                            },
                        ));
                    }
                }
                ModuleDecl::ExportDefaultDecl(decl) => {
                    if let DefaultDecl::Fn(fn_expr) = &mut decl.decl {
                        if fn_expr.ident.is_none() {
                            fn_expr.ident = Some(private_ident!(DEFAULT_COMPONENT_NAME));
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mako_core::swc_ecma_visit::VisitMutWith;
    use swc_core::common::GLOBALS;

    use super::NamedExportDefault;
    use crate::ast_2::tests::TestUtils;

    #[test]
    fn test_normal() {
        assert_eq!(
            run(r#"export default function(){}"#),
            "export default function Component$$() {}"
        );
    }

    #[test]
    fn test_conflicts() {
        assert_eq!(
            run(r#"export default function(){} let Component$$ = 1; Component$$ +=1;"#),
            "export default function Component$$() {}\nlet Component$$1 = 1;\nComponent$$1 += 1;"
        );
        assert_eq!(
            run(r#"let Component$$ = 1;export default function(){} Component$$ += 1;"#),
            "let Component$$ = 1;\nexport default function Component$$1() {}\nComponent$$ += 1;"
        );
    }

    #[test]
    fn test_arrow_function() {
        assert_eq!(
            run(r#"export default ()=>{}"#),
            "export default function Component$$() {}"
        );
    }

    #[test]
    fn test_arrow_function_exclude_cases() {
        assert_eq!(
            run(r#"export default async ()=>{};"#),
            "export default async ()=>{};"
        );
    }

    #[test]
    fn test_arrow_function_conflict() {
        assert_eq!(
            run(r#"let Component$$=1;export default ()=>{};Component$$+=1;"#),
            "let Component$$ = 1;\nexport default function Component$$1() {}\nComponent$$ += 1;"
        );
        assert_eq!(
            run(r#"export default ()=>{};let Component$$=1;Component$$+=1;"#),
            "export default function Component$$() {}\nlet Component$$1 = 1;\nComponent$$1 += 1;"
        );
    }

    fn run(js_code: &str) -> String {
        let mut test_utils = TestUtils::gen_js_ast(js_code.to_string());
        let ast = test_utils.ast.js_mut();
        GLOBALS.set(&test_utils.context.meta.script.globals, || {
            let mut visitor = NamedExportDefault::new();
            ast.ast.visit_mut_with(&mut visitor);
        });
        test_utils.js_ast_to_code()
    }
}