use log::error;

use super::FreePrimFn;
use super::Prelude;
use super::free_impl;
use crate::cfg::prelude::setup::default_free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::GenRepr;
use crate::syntax::ParseRepr;
use crate::syntax::ReprError;
use crate::syntax::generate_pretty;
use crate::type_::List;
use crate::type_::Pair;
use crate::type_::Task;
use crate::type_::Text;

#[derive(Clone)]
pub struct SyntaxPrelude {
    pub parse: FreePrimFuncVal,
    pub generate: FreePrimFuncVal,
}

impl Default for SyntaxPrelude {
    fn default() -> Self {
        SyntaxPrelude { parse: parse(), generate: generate() }
    }
}

impl Prelude for SyntaxPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.parse.put(ctx);
        self.generate.put(ctx);
    }
}

pub fn parse() -> FreePrimFuncVal {
    FreePrimFn { id: "syntax.parse", f: free_impl(fn_parse), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "syntax.generate", f: free_impl(fn_generate), mode: default_free_mode() }
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
            Val::Task(task) => {
                let func = (&task.func).try_into()?;
                let ctx = (&task.ctx).try_into()?;
                let input = (&task.input).try_into()?;
                GenRepr::Task(Box::new(Task { action: task.action, func, ctx, input }))
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
