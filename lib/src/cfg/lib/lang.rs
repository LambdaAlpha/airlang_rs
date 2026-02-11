use std::rc::Rc;

use const_format::concatcp;

use super::FreeImpl;
use super::ImplExtra;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::Id;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Cell;
use crate::type_::Text;

#[derive(Clone)]
pub struct LangLib {
    pub data: PrimFuncVal,
    pub id: PrimFuncVal,
    pub code: PrimFuncVal,
    pub eval: PrimFuncVal,
    pub parse: PrimFuncVal,
    pub generate: PrimFuncVal,
}

const LANGUAGE: &str = "language";

pub const DATA: &str = concatcp!(PREFIX_ID, LANGUAGE, ".semantics.data");
pub const ID: &str = concatcp!(PREFIX_ID, LANGUAGE, ".semantics.id");
pub const CODE: &str = concatcp!(PREFIX_ID, LANGUAGE, ".semantics.code");
pub const EVAL: &str = concatcp!(PREFIX_ID, LANGUAGE, ".semantics.eval");
pub const PARSE: &str = concatcp!(PREFIX_ID, LANGUAGE, ".syntax.parse");
pub const GENERATE: &str = concatcp!(PREFIX_ID, LANGUAGE, ".syntax.generate");

impl Default for LangLib {
    fn default() -> Self {
        LangLib {
            data: data(),
            id: id(),
            code: code(),
            eval: eval(),
            parse: parse(),
            generate: generate(),
        }
    }
}

impl CfgMod for LangLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, DATA, self.data);
        extend_func(cfg, ID, self.id);
        extend_func(cfg, CODE, self.code);
        extend_func(cfg, EVAL, self.eval);
        extend_func(cfg, PARSE, self.parse);
        extend_func(cfg, GENERATE, self.generate);
    }
}

pub fn data() -> PrimFuncVal {
    PrimFunc { raw_input: true, fn_: Rc::new(Id), ctx: PrimCtx::Free }.into()
}

pub fn id() -> PrimFuncVal {
    PrimFunc { raw_input: false, fn_: Rc::new(Id), ctx: PrimCtx::Free }.into()
}

pub fn code() -> PrimFuncVal {
    PrimFunc { raw_input: true, fn_: Rc::new(Eval), ctx: PrimCtx::Mut }.into()
}

pub fn eval() -> PrimFuncVal {
    PrimFunc { raw_input: false, fn_: Rc::new(Eval), ctx: PrimCtx::Mut }.into()
}

pub fn parse() -> PrimFuncVal {
    FreeImpl { fn_: fn_parse }.build(ImplExtra { raw_input: false })
}

fn fn_parse(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(input) = input else {
        return bug!(cfg, "{PARSE}: expected input to be a text, but got {input}");
    };
    let Ok(val) = crate::syntax::parse(&input) else {
        return Val::default();
    };
    Val::Cell(Cell::new(val).into())
}

pub fn generate() -> PrimFuncVal {
    FreeImpl { fn_: fn_generate }.build(ImplExtra { raw_input: false })
}

fn fn_generate(_cfg: &mut Cfg, input: Val) -> Val {
    let str = format!("{input:#}");
    Val::Text(Text::from(str).into())
}
