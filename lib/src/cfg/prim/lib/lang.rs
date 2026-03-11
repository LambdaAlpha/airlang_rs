use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::func::PrimInput;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Cell;
use crate::type_::Text;
use crate::utils::memory::leak_const;

#[derive(Clone)]
pub struct LangLib {
    pub eval: PrimFuncVal,
    pub parse: PrimFuncVal,
    pub generate: PrimFuncVal,
}

const LANGUAGE: &str = "language";

pub const EVAL: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".semantics.eval");
pub const PARSE: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.parse");
pub const GENERATE: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.generate");

impl Default for LangLib {
    fn default() -> Self {
        LangLib {
            eval: PrimFunc { fn_: leak_const(Eval), ctx: PrimCtx::Mut, input: PrimInput::Aware }
                .into(),
            parse: CtxFreeInputAwareFunc { fn_: parse }.build(),
            generate: CtxFreeInputAwareFunc { fn_: generate }.build(),
        }
    }
}

impl CfgMod for LangLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, EVAL, self.eval);
        extend_func(cfg, PARSE, self.parse);
        extend_func(cfg, GENERATE, self.generate);
    }
}

pub fn parse(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(input) = input else {
        return bug!(cfg, "{PARSE}: expected input to be a text, but got {input}");
    };
    let Ok(val) = input.parse() else {
        return Val::default();
    };
    Val::Cell(Cell::new(val).into())
}

pub fn generate(_cfg: &mut Cfg, input: Val) -> Val {
    let str = format!("{input:#}");
    Val::Text(Text::from(str).into())
}
