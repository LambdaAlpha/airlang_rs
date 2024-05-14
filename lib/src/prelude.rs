use std::rc::Rc;

use crate::{
    ctx::{
        Ctx,
        CtxMap,
        CtxValue,
    },
    func::{
        CtxConstFn,
        CtxFreeFn,
        CtxMutableFn,
        Func,
        FuncImpl,
        FuncTransformer,
        Primitive,
    },
    mode::{
        ListMode,
        MapMode,
        Mode,
        ValMode,
    },
    prelude::{
        annotation::AnnotationPrelude,
        answer::AnswerPrelude,
        ask::AskPrelude,
        assert::AssertPrelude,
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
    Ask,
    AskMode,
    Call,
    CallMode,
    List,
    ListItemMode,
    Map,
    Pair,
    SymbolMode,
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
    pub(crate) ask: AskPrelude,
    pub(crate) assert: AssertPrelude,
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
    fn put(&self, m: &mut CtxMap) {
        self.meta.put(m);
        self.syntax.put(m);
        self.value.put(m);
        self.ctx.put(m);
        self.ctrl.put(m);
        self.transform.put(m);
        self.logic.put(m);
        self.func.put(m);
        self.call.put(m);
        self.ask.put(m);
        self.assert.put(m);
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
    let ctx_map = PRELUDE.with(|prelude| {
        let mut m = CtxMap::default();
        prelude.put(&mut m);
        m
    });
    Ctx {
        map: ctx_map,
        meta: None,
    }
}

pub(crate) trait Prelude {
    fn put(&self, m: &mut CtxMap);
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
    pub(crate) fn put(&self, m: &mut CtxMap) {
        let name = Symbol::from_str(self.name);
        let value = CtxValue::new_const(self.value.clone().into());
        m.insert(name, value);
    }
}

fn named_free_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxFreeFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<Rc<dyn CtxFreeFn>>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncTransformer::Free(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxConstFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<Rc<dyn CtxConstFn>>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncTransformer::Const(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxMutableFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<Rc<dyn CtxMutableFn>>::new(name, func);
    let func = Func::new(
        input_mode,
        output_mode,
        FuncTransformer::Mutable(FuncImpl::Primitive(primitive)),
    );
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn default_mode() -> Mode {
    Mode::default()
}

fn symbol_id_mode() -> Mode {
    let mode = ValMode {
        symbol: SymbolMode::Id,
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn pair_mode(first: Mode, second: Mode) -> Mode {
    let mode = ValMode {
        pair: Box::new(Pair::new(first, second)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn call_mode(func: Mode, input: Mode) -> Mode {
    let mode = ValMode {
        call: Box::new(CallMode::Struct(Call::new(func, input))),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn ask_mode(func: Mode, output: Mode) -> Mode {
    let mode = ValMode {
        ask: Box::new(AskMode::Struct(Ask::new(func, output))),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn list_all_mode(mode: Mode) -> Mode {
    let mode = ValMode {
        list: Box::new(ListMode::All(mode)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn list_some_mode(list_item: List<ListItemMode>) -> Mode {
    let mode = ValMode {
        list: Box::new(ListMode::Some(list_item)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn map_all_mode(key: Mode, value: Mode) -> Mode {
    let mode = ValMode {
        map: Box::new(MapMode::All(Pair::new(key, value))),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn map_some_mode(map_mode: Map<Val, Mode>) -> Mode {
    let mode = ValMode {
        map: Box::new(MapMode::Some(map_mode)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
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

mod ask;

mod assert;

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
