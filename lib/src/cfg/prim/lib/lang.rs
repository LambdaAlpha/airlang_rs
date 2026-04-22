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
use crate::type_::Bit;
use crate::type_::Cell;
use crate::type_::Key;
use crate::type_::Text;
use crate::utils::memory::leak_const;

#[derive(Copy, Clone)]
pub struct LangLib {
    pub semantics_eval: PrimFuncVal,
    pub syntax_parse: PrimFuncVal,
    pub syntax_generate_pretty: PrimFuncVal,
    pub syntax_generate_key: PrimFuncVal,
    pub syntax_is_valid: PrimFuncVal,
}

const LANGUAGE: &str = "language";

pub const SEMANTICS_EVAL: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".semantics.eval");
pub const SYNTAX_PARSE: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.parse");
pub const SYNTAX_GENERATE_PRETTY: &str =
    concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.generate_pretty");
pub const SYNTAX_GENERATE_KEY: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.generate_key");
pub const SYNTAX_IS_VALID: &str = concatcp!(PREFIX_CELL, LANGUAGE, ".syntax.is_valid");

impl Default for LangLib {
    fn default() -> Self {
        Self {
            semantics_eval: PrimFunc {
                fn_: leak_const(Eval),
                ctx: PrimCtx::Mut,
                input: PrimInput::Aware,
            }
            .into(),
            syntax_parse: CtxFreeInputAwareFunc { fn_: syntax_parse }.build(),
            syntax_generate_pretty: CtxConstInputFreeFunc { fn_: syntax_generate_pretty }.build(),
            syntax_generate_key: CtxConstInputFreeFunc { fn_: syntax_generate_key }.build(),
            syntax_is_valid: CtxConstInputFreeFunc { fn_: syntax_is_valid }.build(),
        }
    }
}

impl CfgMod for LangLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, SEMANTICS_EVAL, self.semantics_eval);
        extend_func(cfg, SYNTAX_PARSE, self.syntax_parse);
        extend_func(cfg, SYNTAX_GENERATE_PRETTY, self.syntax_generate_pretty);
        extend_func(cfg, SYNTAX_GENERATE_KEY, self.syntax_generate_key);
        extend_func(cfg, SYNTAX_IS_VALID, self.syntax_is_valid);
    }
}

pub fn syntax_parse(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(input) = input else {
        return bug!(cfg, "{SYNTAX_PARSE}: expected input to be a text, but got {input}");
    };
    let Ok(val) = input.parse() else {
        return Val::default();
    };
    Val::Cell(Cell::new(val).into())
}

pub fn syntax_generate_pretty(_cfg: &mut Cfg, val: &Val) -> Val {
    let str = format!("{val:#}");
    Val::Text(Text::from(str).into())
}

pub fn syntax_generate_key(_cfg: &mut Cfg, val: &Val) -> Val {
    let mut str = String::new();
    let options = FmtOptions { id_mode: true, pretty: false, ..Default::default() };
    val.fmt(options, &mut str).unwrap();
    Val::Key(Key::from_string_unchecked(str))
}

pub fn syntax_is_valid(_cfg: &mut Cfg, val: &Val) -> Val {
    Val::Bit(Bit::from(is_syntax(val)))
}

fn is_syntax(val: &Val) -> bool {
    match val {
        Val::Unit(_)
        | Val::Bit(_)
        | Val::Key(_)
        | Val::Text(_)
        | Val::Int(_)
        | Val::Decimal(_)
        | Val::Byte(_) => true,
        Val::Cell(cell) => is_syntax(&cell.value),
        Val::Pair(pair) => is_syntax(&pair.left) && is_syntax(&pair.right),
        Val::List(list) => list.iter().all(is_syntax),
        Val::Map(map) => map.values().all(is_syntax),
        Val::Quote(quote) => is_syntax(&quote.value),
        Val::Call(call) => is_syntax(&call.func) && is_syntax(&call.input),
        Val::Solve(solve) => is_syntax(&solve.func) && is_syntax(&solve.output),
        Val::Link(_) | Val::Cfg(_) | Val::Func(_) | Val::Dyn(_) => false,
    }
}
