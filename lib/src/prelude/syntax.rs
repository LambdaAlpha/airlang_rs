use log::error;

use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
use super::free_impl;
use crate::prelude::setup::default_free_mode;
use crate::semantics::val::ByteVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::IntVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::NumberVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::TextVal;
use crate::semantics::val::Val;
use crate::syntax::GenRepr;
use crate::syntax::ParseRepr;
use crate::syntax::ReprError;
use crate::syntax::generate_pretty;
use crate::syntax::repr::ListRepr;
use crate::syntax::repr::MapRepr;
use crate::syntax::repr::PairRepr;
use crate::syntax::repr::Repr;
use crate::syntax::repr::TaskRepr;
use crate::type_::Byte;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Task;
use crate::type_::Text;

#[derive(Clone)]
pub struct SyntaxPrelude {
    pub parse: FreeStaticPrimFuncVal,
    pub generate: FreeStaticPrimFuncVal,
}

impl Default for SyntaxPrelude {
    fn default() -> Self {
        SyntaxPrelude { parse: parse(), generate: generate() }
    }
}

impl Prelude for SyntaxPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.parse.put(ctx);
        self.generate.put(ctx);
    }
}

pub fn parse() -> FreeStaticPrimFuncVal {
    FreeFn { id: "syntax.parse", f: free_impl(fn_parse), mode: default_free_mode() }.free_static()
}

fn fn_parse(input: Val) -> Val {
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

pub fn generate() -> FreeStaticPrimFuncVal {
    FreeFn { id: "syntax.generate", f: free_impl(fn_generate), mode: default_free_mode() }
        .free_static()
}

fn fn_generate(input: Val) -> Val {
    let Ok(repr) = (&input).try_into() else {
        error!("generate {input:?} failed");
        return Val::default();
    };
    let str = generate_pretty(repr);
    Val::Text(Text::from(str).into())
}

// todo impl derive from
impl From<&Repr> for Val {
    fn from(value: &Repr) -> Self {
        match value {
            Repr::Unit(unit) => Val::Unit(*unit),
            Repr::Bit(bit) => Val::Bit(*bit),
            Repr::Symbol(symbol) => Val::Symbol(symbol.clone()),
            Repr::Text(text) => Val::Text(TextVal::from(text.clone())),
            Repr::Int(int) => Val::Int(IntVal::from(int.clone())),
            Repr::Number(number) => Val::Number(NumberVal::from(number.clone())),
            Repr::Byte(byte) => Val::Byte(ByteVal::from(byte.clone())),
            Repr::Pair(pair) => Val::Pair(PairVal::from(&**pair)),
            Repr::Task(task) => Val::Task(TaskVal::from(&**task)),
            Repr::List(list) => Val::List(ListVal::from(list)),
            Repr::Map(map) => Val::Map(MapVal::from(map)),
        }
    }
}

impl From<Repr> for Val {
    fn from(value: Repr) -> Self {
        match value {
            Repr::Unit(unit) => Val::Unit(unit),
            Repr::Bit(bit) => Val::Bit(bit),
            Repr::Symbol(symbol) => Val::Symbol(symbol),
            Repr::Text(text) => Val::Text(TextVal::from(text)),
            Repr::Int(int) => Val::Int(IntVal::from(int)),
            Repr::Number(number) => Val::Number(NumberVal::from(number)),
            Repr::Byte(byte) => Val::Byte(ByteVal::from(byte)),
            Repr::Pair(pair) => Val::Pair(PairVal::from(*pair)),
            Repr::Task(task) => Val::Task(TaskVal::from(*task)),
            Repr::List(list) => Val::List(ListVal::from(list)),
            Repr::Map(map) => Val::Map(MapVal::from(map)),
        }
    }
}

impl TryInto<Repr> for &Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(unit) => Ok(Repr::Unit(*unit)),
            Val::Bit(bit) => Ok(Repr::Bit(*bit)),
            Val::Symbol(symbol) => Ok(Repr::Symbol(symbol.clone())),
            Val::Text(text) => Ok(Repr::Text(Text::clone(text))),
            Val::Int(int) => Ok(Repr::Int(Int::clone(int))),
            Val::Number(number) => Ok(Repr::Number(Number::clone(number))),
            Val::Byte(byte) => Ok(Repr::Byte(Byte::clone(byte))),
            Val::Pair(pair) => Ok(Repr::Pair(Box::new(pair.try_into()?))),
            Val::Task(task) => Ok(Repr::Task(Box::new(task.try_into()?))),
            Val::List(list) => Ok(Repr::List(list.try_into()?)),
            Val::Map(map) => Ok(Repr::Map(map.try_into()?)),
            _ => Err(ReprError {}),
        }
    }
}

impl TryInto<Repr> for Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(unit) => Ok(Repr::Unit(unit)),
            Val::Bit(bit) => Ok(Repr::Bit(bit)),
            Val::Symbol(symbol) => Ok(Repr::Symbol(symbol)),
            Val::Text(text) => Ok(Repr::Text(text.into())),
            Val::Int(int) => Ok(Repr::Int(int.into())),
            Val::Number(number) => Ok(Repr::Number(number.into())),
            Val::Byte(byte) => Ok(Repr::Byte(byte.into())),
            Val::Pair(pair) => Ok(Repr::Pair(Box::new(pair.try_into()?))),
            Val::Task(task) => Ok(Repr::Task(Box::new(task.try_into()?))),
            Val::List(list) => Ok(Repr::List(list.try_into()?)),
            Val::Map(map) => Ok(Repr::Map(map.try_into()?)),
            _ => Err(ReprError {}),
        }
    }
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

impl From<&PairRepr> for PairVal {
    fn from(value: &PairRepr) -> Self {
        let pair = Pair::new(Val::from(&value.first), Val::from(&value.second));
        Self::new(Box::new(pair))
    }
}

impl From<PairRepr> for PairVal {
    fn from(value: PairRepr) -> Self {
        let pair = Pair::new(Val::from(value.first), Val::from(value.second));
        Self::new(Box::new(pair))
    }
}

impl TryInto<PairRepr> for &PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        Ok(PairRepr::new((&self.first).try_into()?, (&self.second).try_into()?))
    }
}

impl TryInto<PairRepr> for PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        let pair = self.unwrap();
        let pair = PairRepr::new(pair.first.try_into()?, pair.second.try_into()?);
        Ok(pair)
    }
}

impl From<&TaskRepr> for TaskVal {
    fn from(value: &TaskRepr) -> Self {
        Self::new(Box::new(Task {
            action: value.action,
            func: Val::from(&value.func),
            ctx: Val::from(&value.ctx),
            input: Val::from(&value.input),
        }))
    }
}

impl From<TaskRepr> for TaskVal {
    fn from(value: TaskRepr) -> Self {
        Self::new(Box::new(Task {
            action: value.action,
            func: Val::from(value.func),
            ctx: Val::from(value.ctx),
            input: Val::from(value.input),
        }))
    }
}

impl TryInto<TaskRepr> for &TaskVal {
    type Error = ReprError;
    fn try_into(self) -> Result<TaskRepr, Self::Error> {
        Ok(Task {
            action: self.action,
            func: (&self.func).try_into()?,
            ctx: (&self.ctx).try_into()?,
            input: (&self.input).try_into()?,
        })
    }
}

impl TryInto<TaskRepr> for TaskVal {
    type Error = ReprError;
    fn try_into(self) -> Result<TaskRepr, Self::Error> {
        let task = self.unwrap();
        Ok(Task {
            action: task.action,
            func: task.func.try_into()?,
            ctx: task.ctx.try_into()?,
            input: task.input.try_into()?,
        })
    }
}

impl From<&ListRepr> for ListVal {
    fn from(value: &ListRepr) -> Self {
        let list = value.iter().map(Into::into).collect();
        Self::new(Box::new(list))
    }
}

impl From<ListRepr> for ListVal {
    fn from(value: ListRepr) -> Self {
        let list = value.into_iter().map(Into::into).collect();
        Self::new(Box::new(list))
    }
}

impl TryInto<ListRepr> for &ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.iter().map(TryInto::try_into).collect()
    }
}

impl TryInto<ListRepr> for ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.unwrap().into_iter().map(TryInto::try_into).collect()
    }
}

impl From<&MapRepr> for MapVal {
    fn from(value: &MapRepr) -> Self {
        let map = value.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self::new(Box::new(map))
    }
}

impl From<MapRepr> for MapVal {
    fn from(value: MapRepr) -> Self {
        let map = value.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self::new(Box::new(map))
    }
}

impl TryInto<MapRepr> for &MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.iter().map(|(k, v)| Ok((k.try_into()?, v.try_into()?))).collect()
    }
}

impl TryInto<MapRepr> for MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.unwrap().into_iter().map(|(k, v)| Ok((k.try_into()?, v.try_into()?))).collect()
    }
}
