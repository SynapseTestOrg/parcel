use std::collections::HashSet;

use swc_atoms::JsWord;
use swc_ecmascript::ast::{Expr, Lit, Str, StrKind, UnaryOp};
use swc_ecmascript::visit::{Fold, FoldWith};

use crate::id;
use crate::utils::IdentId;

pub struct TypeofReplacer<'a> {
  pub decls: &'a HashSet<IdentId>,
}

impl<'a> Fold for TypeofReplacer<'a> {
  fn fold_expr(&mut self, node: Expr) -> Expr {
    if let Expr::Unary(ref unary) = node {
      // typeof require -> "function"
      // typeof module -> "object"
      if unary.op == UnaryOp::TypeOf {
        if let Expr::Ident(ident) = &*unary.arg {
          if ident.sym == js_word!("require") && !self.decls.contains(&id!(ident)) {
            return Expr::Lit(Lit::Str(Str {
              kind: StrKind::Synthesized,
              has_escape: false,
              span: unary.span,
              value: js_word!("function"),
            }));
          }
          let exports: JsWord = "exports".into();
          if ident.sym == exports && !self.decls.contains(&id!(ident)) {
            return Expr::Lit(Lit::Str(Str {
              kind: StrKind::Synthesized,
              has_escape: false,
              span: unary.span,
              value: js_word!("object"),
            }));
          }

          if ident.sym == js_word!("module") && !self.decls.contains(&id!(ident)) {
            return Expr::Lit(Lit::Str(Str {
              kind: StrKind::Synthesized,
              has_escape: false,
              span: unary.span,
              value: js_word!("object"),
            }));
          }
        }
      }
    }
    node.fold_children_with(self)
  }
}
