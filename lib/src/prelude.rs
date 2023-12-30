use crate::{
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
        Func,
        FuncEval,
        FuncImpl,
        Primitive,
    },
    io_mode::{
        IoMode,
        ListMode,
        MapMode,
        MatchMode,
    },
    prelude::{
        bool::BoolPrelude,
        bytes::BytesPrelude,
        call::CallPrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
        eval::EvalPrelude,
        extension::ExtPrelude,
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
    symbol::Symbol,
    types::refer::Reader,
    val::{
        func::FuncVal,
        Val,
    },
    Call,
    CallMode,
    EvalMode,
    List,
    ListItemMode,
    Map,
    Pair,
    PairMode,
    Reverse,
    ReverseMode,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    pub(crate) meta: MetaPrelude,
    pub(crate) syntax: SyntaxPrelude,
    pub(crate) value: ValuePrelude,
    pub(crate) ctx: CtxPrelude,
    pub(crate) ctrl: CtrlPrelude,
    pub(crate) eval: EvalPrelude,
    pub(crate) logic: LogicPrelude,
    pub(crate) func: FuncPrelude,
    pub(crate) call: CallPrelude,
    pub(crate) prop: PropPrelude,
    pub(crate) symbol: SymbolPrelude,
    pub(crate) unit: UnitPrelude,
    pub(crate) bool: BoolPrelude,
    pub(crate) int: IntPrelude,
    pub(crate) float: FloatPrelude,
    pub(crate) bytes: BytesPrelude,
    pub(crate) str: StrPrelude,
    pub(crate) pair: PairPrelude,
    pub(crate) list: ListPrelude,
    pub(crate) map: MapPrelude,
    pub(crate) extension: ExtPrelude,
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
        self.extension.put(m);
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

pub(crate) trait Prelude {
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
    input_mode: IoMode,
    output_mode: IoMode,
    func: impl Fn(Val) -> Val + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CtxFreeFn>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncEval::Free(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Reader::new(func));
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: IoMode,
    output_mode: IoMode,
    func: impl Fn(CtxForConstFn, Val) -> Val + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CtxConstFn>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncEval::Const(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Reader::new(func));
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: IoMode,
    output_mode: IoMode,
    func: impl Fn(CtxForMutableFn, Val) -> Val + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<CtxMutableFn>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncEval::Mutable(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Reader::new(func));
    Named::new(name, func_val)
}

fn default_mode() -> IoMode {
    IoMode::default()
}

fn symbol_value_mode() -> IoMode {
    let mode = MatchMode {
        symbol: EvalMode::Value,
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn pair_mode(first: IoMode, second: IoMode) -> IoMode {
    let mode = MatchMode {
        pair: Box::new(PairMode::Pair(Pair::new(first, second))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn call_mode(func: IoMode, input: IoMode) -> IoMode {
    let mode = MatchMode {
        call: Box::new(CallMode::Call(Call::new(func, input))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn reverse_mode(func: IoMode, output: IoMode) -> IoMode {
    let mode = MatchMode {
        reverse: Box::new(ReverseMode::Reverse(Reverse::new(func, output))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn list_mode(list_mode: ListMode) -> IoMode {
    let mode = MatchMode {
        list: Box::new(list_mode),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn list_mode_for_all(mode: IoMode) -> IoMode {
    let mode = MatchMode {
        list: Box::new(ListMode::ForAll(mode)),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn list_mode_for_some(list_item: List<ListItemMode>) -> IoMode {
    let mode = MatchMode {
        list: Box::new(ListMode::ForSome(list_item)),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn map_mode(map_mode: MapMode) -> IoMode {
    let mode = MatchMode {
        map: Box::new(map_mode),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn map_mode_for_all(key: IoMode, value: IoMode) -> IoMode {
    let mode = MatchMode {
        map: Box::new(MapMode::ForAll(Pair::new(key, value))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn map_mode_for_some(map_mode: Map<Val, IoMode>) -> IoMode {
    let mode = MatchMode {
        map: Box::new(MapMode::ForSome(map_mode)),
        ..Default::default()
    };
    IoMode::Match(mode)
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

mod extension;

mod utils;
