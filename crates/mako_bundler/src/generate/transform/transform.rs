use std::collections::HashMap;

use swc_common::collections::AHashMap;
use swc_common::sync::Lrc;
use swc_common::DUMMY_SP;
use swc_common::{Globals, SourceMap, GLOBALS};
use swc_ecma_ast::{Expr, Lit, Module, Str};
use swc_ecma_codegen::text_writer::JsWriter;
use swc_ecma_codegen::Emitter;
use swc_ecma_transforms::helpers::{Helpers, HELPERS};
use swc_ecma_visit::VisitMutWith;

use crate::context::Context;
use crate::module::ModuleAst;

use super::dep_replacer::DepReplacer;
use super::env_replacer::EnvReplacer;

pub struct TransformParam<'a> {
    pub ast: &'a ModuleAst,
    pub cm: &'a Lrc<SourceMap>,
    pub dep_map: HashMap<String, String>,
    pub env_map: HashMap<String, String>,
}

pub struct TransformResult {
    pub ast: Module,
    pub code: String,
}

pub fn transform(transform_param: &TransformParam, _context: &Context) -> TransformResult {
    let globals = Globals::default();

    let module_ast = if let ModuleAst::Script(ast) = transform_param.ast {
        ast
    } else {
        panic!("not support module")
    };

    let mut env_map = AHashMap::default();
    transform_param
        .env_map
        .clone()
        .into_iter()
        .for_each(|(k, v)| {
            env_map.insert(
                k.into(),
                Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    raw: None,
                    value: v.into(),
                })),
            );
        });

    let mut ast = module_ast.clone();
    let cm = transform_param.cm.clone();
    GLOBALS.set(&globals, || {
        let helpers = Helpers::new(true);
        HELPERS.set(&helpers, || {
            let mut dep_replacer = DepReplacer {
                dep_map: transform_param.dep_map.clone(),
            };
            ast.visit_mut_with(&mut dep_replacer);

            let mut env_replacer = EnvReplacer::new(Lrc::new(env_map));
            ast.visit_mut_with(&mut env_replacer);
        });
    });

    // ast to code
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm: cm.clone(),
            comments: None,
            wr: Box::new(JsWriter::new(cm, "\n", &mut buf, None)),
        };
        emitter.emit_module(&ast).unwrap();
    }
    let code = String::from_utf8(buf).unwrap();
    // println!("code: {}", code);

    TransformResult { ast, code }
}