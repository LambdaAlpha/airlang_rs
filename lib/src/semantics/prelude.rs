use crate::{
    semantics::{
        ctx::{
            Ctx,
            NameMap,
            TaggedVal,
        },
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
        },
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            FuncEval,
            FuncImpl,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            bool::BoolPrelude,
            bytes::BytesPrelude,
            call::CallPrelude,
            ctrl::CtrlPrelude,
            ctx::CtxPrelude,
            eval::EvalPrelude,
            float::FloatPrelude,
            func::FuncPrelude,
            int::IntPrelude,
            list::ListPrelude,
            logic::LogicPrelude,
            map::MapPrelude,
            meta::MetaPrelude,
            pair::PairPrelude,
            prop::PropPrelude,
            str::StrPrelude,
            symbol::SymbolPrelude,
            syntax::SyntaxPrelude,
            unit::UnitPrelude,
            value::ValuePrelude,
        },
        val::{
            FuncVal,
            Val,
        },
        Func,
    },
    types::{
        Reader,
        Symbol,
    },
};

thread_local! (pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    meta: MetaPrelude,
    syntax: SyntaxPrelude,
    value: ValuePrelude,
    ctx: CtxPrelude,
    ctrl: CtrlPrelude,
    eval: EvalPrelude,
    logic: LogicPrelude,
    func: FuncPrelude,
    call: CallPrelude,
    prop: PropPrelude,
    symbol: SymbolPrelude,
    unit: UnitPrelude,
    bool: BoolPrelude,
    int: IntPrelude,
    float: FloatPrelude,
    bytes: BytesPrelude,
    str: StrPrelude,
    pair: PairPrelude,
    list: ListPrelude,
    map: MapPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, m: &mut NameMap) {
        self.meta.put(m);
        self.syntax.put(m);
        self.value.put(m);
        self.ctx.put(m);
        self.ctrl.put(m);
        self.eval.put(m);
        self.logic.put(m);
        self.func.put(m);
        self.call.put(m);
        self.prop.put(m);
        self.symbol.put(m);
        self.unit.put(m);
        self.bool.put(m);
        self.int.put(m);
        self.float.put(m);
        self.bytes.put(m);
        self.str.put(m);
        self.pair.put(m);
        self.list.put(m);
        self.map.put(m);
    }
}

pub(crate) fn initial_ctx() -> Ctx {
    let name_map = PRELUDE.with(|prelude| {
        let mut m = NameMap::default();
        prelude.put(&mut m);
        m
    });
    Ctx {
        name_map,
        meta: None,
    }
}

trait Prelude {
    fn put(&self, m: &mut NameMap);
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
    pub(crate) fn put(&self, m: &mut NameMap) {
        let name = Symbol::from_str(self.name);
        let value = TaggedVal::new_const(self.value.clone().into());
        m.insert(name, value);
    }
}

fn named_free_fn(
    name: &'static str,
    input_mode: InputMode,
    func: impl Fn(Val) -> Val + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CtxFreeFn>::new(name, func);
    let func = Func::new(input_mode, FuncEval::Free(FuncImpl::Primitive(primitive)));
    let func_val = FuncVal(Reader::new(func));
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: InputMode,
    func: impl Fn(CtxForConstFn, Val) -> Val + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CtxConstFn>::new(name, func);
    let func = Func::new(input_mode, FuncEval::Const(FuncImpl::Primitive(primitive)));
    let func_val = FuncVal(Reader::new(func));
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: InputMode,
    func: impl Fn(CtxForMutableFn, Val) -> Val + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CtxMutableFn>::new(name, func);
    let func = Func::new(
        input_mode,
        FuncEval::Mutable(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Reader::new(func));
    Named::new(name, func_val)
}

mod meta;

mod syntax;

mod value;

mod ctx;

mod ctrl;

mod eval;

mod logic;

mod func;

mod call;

mod reverse;

mod prop;

mod symbol;

mod unit;

mod bool;

mod int;

mod float;

mod bytes;

mod str;

mod pair;

mod list;

mod map;

mod utils;
