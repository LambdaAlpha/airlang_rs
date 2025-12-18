use log::error;

use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_input;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::Id;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::GenRepr;
use crate::syntax::ParseRepr;
use crate::syntax::ReprError;
use crate::syntax::generate_pretty;
use crate::type_::Call;
use crate::type_::List;
use crate::type_::Pair;
use crate::type_::Text;

#[derive(Clone)]
pub struct LangLib {
    pub data: FreePrimFuncVal,
    pub id: FreePrimFuncVal,
    pub code: MutPrimFuncVal,
    pub eval: MutPrimFuncVal,
    pub parse: FreePrimFuncVal,
    pub generate: FreePrimFuncVal,
}

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
        self.data.extend(cfg);
        self.id.extend(cfg);
        self.code.extend(cfg);
        self.eval.extend(cfg);
        self.parse.extend(cfg);
        self.generate.extend(cfg);
    }
}

pub fn data() -> FreePrimFuncVal {
    FreePrimFn { id: "_language.semantics.data", raw_input: true, f: Id }.free()
}

pub fn id() -> FreePrimFuncVal {
    FreePrimFn { id: "_language.semantics.id", raw_input: false, f: Id }.free()
}

pub fn code() -> MutPrimFuncVal {
    DynPrimFn { id: "_language.semantics.code", raw_input: true, f: Eval }.mut_()
}

pub fn eval() -> MutPrimFuncVal {
    DynPrimFn { id: "_language.semantics.eval", raw_input: false, f: Eval }.mut_()
}

pub fn parse() -> FreePrimFuncVal {
    FreePrimFn { id: "_language.syntax.parse", raw_input: false, f: free_impl(fn_parse) }.free()
}

fn fn_parse(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(input) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    let Ok(val) = crate::syntax::parse(&input) else {
        error!("parse {input:?} failed");
        return illegal_input(cfg);
    };
    val
}

pub fn generate() -> FreePrimFuncVal {
    FreePrimFn { id: "_language.syntax.generate", raw_input: false, f: free_impl(fn_generate) }
        .free()
}

fn fn_generate(cfg: &mut Cfg, input: Val) -> Val {
    let Ok(repr) = (&input).try_into() else {
        error!("generate {input:?} failed");
        return illegal_input(cfg);
    };
    let str = generate_pretty(repr);
    Val::Text(Text::from(str).into())
}

impl ParseRepr for Val {}

impl<'a> TryInto<GenRepr<'a>> for &'a Val {
    type Error = ReprError;

    fn try_into(self) -> Result<GenRepr<'a>, Self::Error> {
        let r = match self {
            Val::Unit(unit) => GenRepr::Unit(unit),
            Val::Bit(bit) => GenRepr::Bit(bit),
            Val::Key(key) => GenRepr::Key(key),
            Val::Text(text) => GenRepr::Text(text),
            Val::Int(int) => GenRepr::Int(int),
            Val::Decimal(decimal) => GenRepr::Decimal(decimal),
            Val::Byte(byte) => GenRepr::Byte(byte),
            Val::Pair(pair) => {
                let first = (&pair.first).try_into()?;
                let second = (&pair.second).try_into()?;
                GenRepr::Pair(Box::new(Pair::new(first, second)))
            }
            Val::Call(call) => {
                let func = (&call.func).try_into()?;
                let input = (&call.input).try_into()?;
                GenRepr::Call(Box::new(Call { func, input }))
            }
            Val::List(list) => {
                let list: List<GenRepr> =
                    list.iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
                GenRepr::List(list)
            }
            Val::Map(map) => {
                let map = map
                    .iter()
                    .map(|(k, v)| {
                        let v = v.try_into()?;
                        Ok((k, v))
                    })
                    .collect::<Result<_, _>>()?;
                GenRepr::Map(map)
            }
            _ => return Err(ReprError {}),
        };
        Ok(r)
    }
}
