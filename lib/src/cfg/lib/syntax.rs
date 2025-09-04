use log::error;

use super::FreePrimFn;
use super::Library;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::mode::FuncMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FreePrimFuncVal;
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
pub struct SyntaxLib {
    pub parse: FreePrimFuncVal,
    pub generate: FreePrimFuncVal,
}

impl Default for SyntaxLib {
    fn default() -> Self {
        SyntaxLib { parse: parse(), generate: generate() }
    }
}

impl CfgMod for SyntaxLib {
    fn extend(self, cfg: &Cfg) {
        self.parse.extend(cfg);
        self.generate.extend(cfg);
    }
}

impl Library for SyntaxLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
}

pub fn parse() -> FreePrimFuncVal {
    FreePrimFn { id: "syntax.parse", f: free_impl(fn_parse), mode: FuncMode::default_mode() }.free()
}

fn fn_parse(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(input) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    let Ok(val) = crate::syntax::parse(&input) else {
        error!("parse {input:?} failed");
        return Val::default();
    };
    val
}

pub fn generate() -> FreePrimFuncVal {
    FreePrimFn { id: "syntax.generate", f: free_impl(fn_generate), mode: FuncMode::default_mode() }
        .free()
}

fn fn_generate(_cfg: &mut Cfg, input: Val) -> Val {
    let Ok(repr) = (&input).try_into() else {
        error!("generate {input:?} failed");
        return Val::default();
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
            Val::Symbol(symbol) => GenRepr::Symbol(symbol),
            Val::Text(text) => GenRepr::Text(text),
            Val::Int(int) => GenRepr::Int(int),
            Val::Number(number) => GenRepr::Number(number),
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
                        let k = k.try_into()?;
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
