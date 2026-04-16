use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::func::PrimInput;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::FmtOptions;
use crate::syntax::FmtRepr;
use crate::syntax::SpaceFmt;
use crate::type_::Cell;
use crate::type_::Key;
use crate::type_::Text;
use crate::utils::memory::leak_const;

#[derive(Clone)]
pub struct LangLib {
    pub eval: PrimFuncVal,
    pub parse: PrimFuncVal,
    pub generate_pretty: PrimFuncVal,
    pub generate_key: PrimFuncVal,
}

const LANGUAGE: &str = "language";

pub const EVAL: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".semantics.eval");
pub const PARSE: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.parse");
pub const GENERATE_PRETTY: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.generate_pretty");
pub const GENERATE_KEY: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.generate_key");

impl Default for LangLib {
    fn default() -> Self {
        LangLib {
            eval: PrimFunc { fn_: leak_const(Eval), ctx: PrimCtx::Mut, input: PrimInput::Aware }
                .into(),
            parse: CtxFreeInputAwareFunc { fn_: parse }.build(),
            generate_pretty: CtxConstInputFreeFunc { fn_: generate_pretty }.build(),
            generate_key: CtxConstInputFreeFunc { fn_: generate_key }.build(),
        }
    }
}

impl CfgMod for LangLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, EVAL, self.eval);
        extend_func(cfg, PARSE, self.parse);
        extend_func(cfg, GENERATE_PRETTY, self.generate_pretty);
        extend_func(cfg, GENERATE_KEY, self.generate_key);
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

pub fn generate_pretty(_cfg: &mut Cfg, val: &Val) -> Val {
    let str = format!("{val:#}");
    Val::Text(Text::from(str).into())
}

pub fn generate_key(_cfg: &mut Cfg, val: &Val) -> Val {
    let mut str = String::new();
    let options = FmtOptions { id_mode: true, space: SpaceFmt::Compact, ..Default::default() };
    val.fmt(options, &mut str).unwrap();
    Val::Key(Key::from_string_unchecked(str))
}
