use crate::{
    CompositeMode,
    List,
    Map,
    Pair,
    PairMode,
    ctx::{
        Ctx,
        CtxValue,
        map::CtxMap,
    },
    func::{
        Func,
        Primitive,
        cell::{
            CellFnExt,
            CellPrimitiveExt,
        },
        const1::{
            ConstFn,
            ConstPrimitiveExt,
        },
        free::{
            FreeFn,
            FreePrimitiveExt,
        },
        mut1::{
            MutFn,
            MutPrimitiveExt,
        },
    },
    mode::{
        Mode,
        list::ListMode,
        map::MapMode,
        primitive::PrimitiveMode,
    },
    prelude::{
        abstract1::AbstractPrelude,
        answer::AnswerPrelude,
        ask::AskPrelude,
        bool::BoolPrelude,
        byte::BytePrelude,
        call::CallPrelude,
        case::CasePrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
        extension::ExtPrelude,
        func::FuncPrelude,
        int::IntPrelude,
        list::ListPrelude,
        map::MapPrelude,
        meta::MetaPrelude,
        number::NumberPrelude,
        pair::PairPrelude,
        symbol::SymbolPrelude,
        syntax::SyntaxPrelude,
        text::TextPrelude,
        unit::UnitPrelude,
        value::ValuePrelude,
    },
    symbol::Symbol,
    val::{
        Val,
        func::{
            CellFuncVal,
            ConstFuncVal,
            FreeFuncVal,
            FuncVal,
            MutFuncVal,
        },
    },
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    pub(crate) unit: UnitPrelude,
    pub(crate) bool: BoolPrelude,
    pub(crate) symbol: SymbolPrelude,
    pub(crate) text: TextPrelude,
    pub(crate) int: IntPrelude,
    pub(crate) number: NumberPrelude,
    pub(crate) byte: BytePrelude,
    pub(crate) pair: PairPrelude,
    pub(crate) call: CallPrelude,
    pub(crate) abstract1: AbstractPrelude,
    pub(crate) ask: AskPrelude,
    pub(crate) list: ListPrelude,
    pub(crate) map: MapPrelude,
    pub(crate) ctx: CtxPrelude,
    pub(crate) func: FuncPrelude,
    pub(crate) case: CasePrelude,
    pub(crate) answer: AnswerPrelude,
    pub(crate) extension: ExtPrelude,
    pub(crate) meta: MetaPrelude,
    pub(crate) syntax: SyntaxPrelude,
    pub(crate) value: ValuePrelude,
    pub(crate) ctrl: CtrlPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.unit.put(m);
        self.bool.put(m);
        self.symbol.put(m);
        self.text.put(m);
        self.int.put(m);
        self.number.put(m);
        self.byte.put(m);
        self.pair.put(m);
        self.call.put(m);
        self.abstract1.put(m);
        self.ask.put(m);
        self.list.put(m);
        self.map.put(m);
        self.ctx.put(m);
        self.func.put(m);
        self.case.put(m);
        self.answer.put(m);
        self.extension.put(m);
        self.meta.put(m);
        self.syntax.put(m);
        self.value.put(m);
        self.ctrl.put(m);
    }
}

pub(crate) fn initial_ctx() -> Ctx {
    let variables = PRELUDE.with(|prelude| {
        let mut m = Map::default();
        prelude.put(&mut m);
        m
    });
    let variables = CtxMap::new(variables, false);
    Ctx::new(variables, None)
}

pub(crate) trait Prelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>);
}

#[derive(Clone)]
pub(crate) struct Named<T> {
    pub(crate) name: &'static str,
    pub(crate) value: T,
}

impl<T> Named<T> {
    pub(crate) fn new(name: &'static str, value: T) -> Named<T> {
        Named { name, value }
    }
}

impl<T: Into<Val> + Clone> Named<T> {
    pub(crate) fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        let name = Symbol::from_str(self.name);
        let value = CtxValue::new(self.value.clone().into());
        m.insert(name, value);
    }
}

#[allow(unused)]
fn named_cell_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl CellFnExt + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CellPrimitiveExt>::new(name, func);
    let func = Func::new_primitive(call_mode, ask_mode, cacheable, primitive);
    let func_val = CellFuncVal::from(func);
    Named::new(name, FuncVal::Cell(func_val))
}

fn named_free_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl FreeFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<FreePrimitiveExt>::new(name, func);
    let func = Func::new_primitive(call_mode, ask_mode, cacheable, primitive);
    let func_val = FreeFuncVal::from(func);
    Named::new(name, FuncVal::Free(func_val))
}

fn named_const_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl ConstFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<ConstPrimitiveExt>::new(name, func);
    let func = Func::new_primitive(call_mode, ask_mode, cacheable, primitive);
    let func_val = ConstFuncVal::from(func);
    Named::new(name, FuncVal::Const(func_val))
}

fn named_mut_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl MutFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<MutPrimitiveExt>::new(name, func);
    let func = Func::new_primitive(call_mode, ask_mode, cacheable, primitive);
    let func_val = MutFuncVal::from(func);
    Named::new(name, FuncVal::Mut(func_val))
}

pub(crate) fn id_mode() -> Mode {
    Mode::Primitive(PrimitiveMode::Id)
}

pub(crate) fn form_mode() -> Mode {
    Mode::Primitive(PrimitiveMode::Form)
}

#[allow(unused)]
pub(crate) fn eval_mode() -> Mode {
    Mode::Primitive(PrimitiveMode::Eval)
}

pub(crate) fn pair_mode(first: Mode, second: Mode, default: PrimitiveMode) -> Mode {
    let mode = CompositeMode {
        pair: PairMode::Form(Pair::new(first, second)),
        ..CompositeMode::from(default)
    };
    Mode::Composite(Box::new(mode))
}

#[allow(unused)]
pub(crate) fn list_mode(head: List<Mode>, tail: Mode, default: PrimitiveMode) -> Mode {
    let mode = CompositeMode {
        list: ListMode::Form { head, tail },
        ..CompositeMode::from(default)
    };
    Mode::Composite(Box::new(mode))
}

pub(crate) fn map_mode(
    some: Map<Val, Mode>,
    key: Mode,
    value: Mode,
    default: PrimitiveMode,
) -> Mode {
    let else1 = Pair::new(key, value);
    let mode = CompositeMode {
        map: MapMode::Form { some, else1 },
        ..CompositeMode::from(default)
    };
    Mode::Composite(Box::new(mode))
}

mod unit;

mod bool;

mod symbol;

mod text;

mod int;

mod number;

mod byte;

mod pair;

mod call;

mod abstract1;

mod ask;

mod list;

mod map;

mod ctx;

mod func;

mod case;

mod answer;

mod extension;

mod meta;

mod syntax;

mod value;

mod ctrl;
