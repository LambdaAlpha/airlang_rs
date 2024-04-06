use std::rc::Rc;

use crate::{
    ctx::{
        Ctx,
        NameMap,
        TaggedVal,
    },
    func::{
        CtxConstFn,
        CtxFreeFn,
        CtxMutableFn,
        Func,
        FuncCore,
        FuncImpl,
        Primitive,
    },
    mode::{
        ListMode,
        MapMode,
        Mode,
        TransformMode,
        ValMode,
    },
    prelude::{
        annotation::AnnotationPrelude,
        answer::AnswerPrelude,
        bool::BoolPrelude,
        bytes::BytesPrelude,
        call::CallPrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
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
        reverse::ReversePrelude,
        str::StrPrelude,
        symbol::SymbolPrelude,
        syntax::SyntaxPrelude,
        transform::TransformPrelude,
        unit::UnitPrelude,
        value::ValuePrelude,
    },
    symbol::Symbol,
    val::{
        func::FuncVal,
        Val,
    },
    Call,
    CallMode,
    List,
    ListItemMode,
    Map,
    Pair,
    Reverse,
    ReverseMode,
    Transform,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    pub(crate) meta: MetaPrelude,
    pub(crate) syntax: SyntaxPrelude,
    pub(crate) value: ValuePrelude,
    pub(crate) ctx: CtxPrelude,
    pub(crate) ctrl: CtrlPrelude,
    pub(crate) transform: TransformPrelude,
    pub(crate) logic: LogicPrelude,
    pub(crate) func: FuncPrelude,
    pub(crate) call: CallPrelude,
    pub(crate) reverse: ReversePrelude,
    pub(crate) prop: PropPrelude,
    pub(crate) answer: AnswerPrelude,
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
    pub(crate) annotation: AnnotationPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, m: &mut NameMap) {
        self.meta.put(m);
        self.syntax.put(m);
        self.value.put(m);
        self.ctx.put(m);
        self.ctrl.put(m);
        self.transform.put(m);
        self.logic.put(m);
        self.func.put(m);
        self.call.put(m);
        self.reverse.put(m);
        self.prop.put(m);
        self.answer.put(m);
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
        self.annotation.put(m);
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
    input_mode: TransformMode,
    output_mode: TransformMode,
    func: impl CtxFreeFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<Box<dyn CtxFreeFn>>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncCore::Free(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Rc::new(func));
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: TransformMode,
    output_mode: TransformMode,
    func: impl CtxConstFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<Box<dyn CtxConstFn>>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncCore::Const(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Rc::new(func));
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: TransformMode,
    output_mode: TransformMode,
    func: impl CtxMutableFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<Box<dyn CtxMutableFn>>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncCore::Mutable(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal(Rc::new(func));
    Named::new(name, func_val)
}

fn default_mode() -> TransformMode {
    TransformMode::default()
}

fn symbol_id_mode() -> TransformMode {
    let mode = ValMode {
        symbol: Transform::Id,
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn pair_mode(first: TransformMode, second: TransformMode) -> TransformMode {
    let mode = ValMode {
        pair: Box::new(Pair::new(first, second)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn call_mode(func: TransformMode, input: TransformMode) -> TransformMode {
    let mode = ValMode {
        call: Mode::new(CallMode::ForAll(Call::new(func, input))),
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn reverse_mode(func: TransformMode, output: TransformMode) -> TransformMode {
    let mode = ValMode {
        reverse: Mode::new(ReverseMode::ForAll(Reverse::new(func, output))),
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn list_for_all_mode(mode: TransformMode) -> TransformMode {
    let mode = ValMode {
        list: Box::new(ListMode::ForAll(mode)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn list_for_some_mode(list_item: List<ListItemMode>) -> TransformMode {
    let mode = ValMode {
        list: Box::new(ListMode::ForSome(list_item)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn map_for_all_mode(key: TransformMode, value: TransformMode) -> TransformMode {
    let mode = ValMode {
        map: Box::new(MapMode::ForAll(Pair::new(key, value))),
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn map_for_some_mode(map_mode: Map<Val, TransformMode>) -> TransformMode {
    let mode = ValMode {
        map: Box::new(MapMode::ForSome(map_mode)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

mod meta;

mod syntax;

mod value;

mod ctx;

mod ctrl;

mod transform;

mod logic;

mod func;

mod call;

mod reverse;

mod prop;

mod answer;

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

mod annotation;
