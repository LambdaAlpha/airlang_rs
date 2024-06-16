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
        annotate::AnnotatePrelude,
        answer::AnswerPrelude,
        ask::AskPrelude,
        assert::AssertPrelude,
        bool::BoolPrelude,
        bytes::BytesPrelude,
        call::CallPrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
        extension::ExtPrelude,
        func::FuncPrelude,
        int::IntPrelude,
        list::ListPrelude,
        logic::LogicPrelude,
        map::MapPrelude,
        meta::MetaPrelude,
        number::NumberPrelude,
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
    List,
    ListItemMode,
    Map,
    Pair,
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
    pub(crate) ask: AskPrelude,
    pub(crate) assert: AssertPrelude,
    pub(crate) answer: AnswerPrelude,
    pub(crate) symbol: SymbolPrelude,
    pub(crate) unit: UnitPrelude,
    pub(crate) bool: BoolPrelude,
    pub(crate) int: IntPrelude,
    pub(crate) number: NumberPrelude,
    pub(crate) bytes: BytesPrelude,
    pub(crate) str: StrPrelude,
    pub(crate) pair: PairPrelude,
    pub(crate) list: ListPrelude,
    pub(crate) map: MapPrelude,
    pub(crate) extension: ExtPrelude,
    pub(crate) annotate: AnnotatePrelude,
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
        self.number.put(m);
        self.bytes.put(m);
        self.str.put(m);
        self.pair.put(m);
        self.list.put(m);
        self.map.put(m);
        self.extension.put(m);
        self.annotate.put(m);
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

fn id_mode() -> Mode {
    Mode {
        default: Transform::Id,
        specialized: None,
    }
}

fn form_mode() -> Mode {
    Mode {
        default: Transform::Form,
        specialized: None,
    }
}

#[allow(unused)]
fn eval_mode() -> Mode {
    Mode {
        default: Transform::Eval,
        specialized: None,
    }
}

fn pair_mode(first: Mode, second: Mode, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        pair: Pair::new(first, second),
        list: ListMode::All(default_mode.clone()),
        map: MapMode::All(Pair::new(default_mode.clone(), default_mode)),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

#[allow(unused)]
fn list_mode(list_item: List<ListItemMode>, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        list: ListMode::Some(list_item),
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        map: MapMode::All(Pair::new(default_mode.clone(), default_mode)),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

fn map_mode(map_mode: Map<Val, Mode>, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        map: MapMode::Some(map_mode),
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        list: ListMode::All(default_mode),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

fn map_all_mode(key: Mode, value: Mode, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        map: MapMode::All(Pair::new(key, value)),
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        list: ListMode::All(default_mode),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
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

mod number;

mod bytes;

mod str;

mod pair;

mod list;

mod map;

mod extension;

mod annotate;
