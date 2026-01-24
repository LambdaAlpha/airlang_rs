use std::rc::Rc;

use const_format::concatcp;
use log::error;

use super::FreeImpl;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::Id;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::GenRepr;
use crate::syntax::ParseRepr;
use crate::syntax::ReprError;
use crate::syntax::generate_pretty;
use crate::type_::Call;
use crate::type_::Cell;
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

pub fn data() -> FreePrimFuncVal {
    FreePrimFunc { raw_input: true, fn_: Rc::new(Id) }.into()
}

pub fn id() -> FreePrimFuncVal {
    FreePrimFunc { raw_input: false, fn_: Rc::new(Id) }.into()
}

pub fn code() -> MutPrimFuncVal {
    MutPrimFunc { raw_input: true, fn_: Rc::new(Eval) }.into()
}

pub fn eval() -> MutPrimFuncVal {
    MutPrimFunc { raw_input: false, fn_: Rc::new(Eval) }.into()
}

pub fn parse() -> FreePrimFuncVal {
    FreeImpl { free: fn_parse }.build()
}

fn fn_parse(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(input) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    let Ok(val) = crate::syntax::parse(&input) else {
        error!("parse {input:?} failed");
        return Val::default();
    };
    Val::Cell(Cell::new(val).into())
}

pub fn generate() -> FreePrimFuncVal {
    FreeImpl { free: fn_generate }.build()
}

fn fn_generate(_cfg: &mut Cfg, input: Val) -> Val {
    let Ok(repr) = (&input).try_into() else {
        error!("generate {input:?} failed");
        return Val::default();
    };
    let str = generate_pretty(repr);
    let text = Val::Text(Text::from(str).into());
    Val::Cell(Cell::new(text).into())
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
            Val::Cell(cell) => {
                let value = (&cell.value).try_into()?;
                GenRepr::Cell(Box::new(Cell::new(value)))
            }
            Val::Pair(pair) => {
                let left = (&pair.left).try_into()?;
                let right = (&pair.right).try_into()?;
                GenRepr::Pair(Box::new(Pair::new(left, right)))
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
