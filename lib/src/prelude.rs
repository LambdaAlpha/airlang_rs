use crate::{
    ctx::{
        map::CtxMap,
        Ctx,
        CtxValue,
    },
    func::{
        const1::{
            ConstFn,
            ConstPrimitiveExt,
        },
        free::{
            FreeFnExt,
            FreePrimitiveExt,
        },
        mut1::{
            MutFn,
            MutPrimitiveExt,
        },
        static1::{
            StaticFn,
            StaticPrimitiveExt,
        },
        Func,
        Primitive,
    },
    mode::{
        basic::BasicMode,
        list::ListMode,
        map::MapMode,
        Mode,
        ValMode,
    },
    prelude::{
        answer::AnswerPrelude,
        ask::AskPrelude,
        bool::BoolPrelude,
        byte::BytePrelude,
        call::CallPrelude,
        case::CasePrelude,
        comment::CommentPrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
        extension::ExtPrelude,
        func::FuncPrelude,
        int::IntPrelude,
        list::ListPrelude,
        map::MapPrelude,
        meta::MetaPrelude,
        mode::ModePrelude,
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
        func::{
            ConstFuncVal,
            FreeFuncVal,
            FuncVal,
            MutFuncVal,
            StaticFuncVal,
        },
        Val,
    },
    List,
    Map,
    Pair,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    pub(crate) meta: MetaPrelude,
    pub(crate) syntax: SyntaxPrelude,
    pub(crate) value: ValuePrelude,
    pub(crate) ctx: CtxPrelude,
    pub(crate) ctrl: CtrlPrelude,
    pub(crate) mode: ModePrelude,
    pub(crate) func: FuncPrelude,
    pub(crate) call: CallPrelude,
    pub(crate) ask: AskPrelude,
    pub(crate) case: CasePrelude,
    pub(crate) answer: AnswerPrelude,
    pub(crate) symbol: SymbolPrelude,
    pub(crate) unit: UnitPrelude,
    pub(crate) bool: BoolPrelude,
    pub(crate) int: IntPrelude,
    pub(crate) number: NumberPrelude,
    pub(crate) byte: BytePrelude,
    pub(crate) text: TextPrelude,
    pub(crate) pair: PairPrelude,
    pub(crate) list: ListPrelude,
    pub(crate) map: MapPrelude,
    pub(crate) extension: ExtPrelude,
    pub(crate) comment: CommentPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.meta.put(m);
        self.syntax.put(m);
        self.value.put(m);
        self.ctx.put(m);
        self.ctrl.put(m);
        self.mode.put(m);
        self.func.put(m);
        self.call.put(m);
        self.ask.put(m);
        self.case.put(m);
        self.answer.put(m);
        self.symbol.put(m);
        self.unit.put(m);
        self.bool.put(m);
        self.int.put(m);
        self.number.put(m);
        self.byte.put(m);
        self.text.put(m);
        self.pair.put(m);
        self.list.put(m);
        self.map.put(m);
        self.extension.put(m);
        self.comment.put(m);
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
fn named_free_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    cacheable: bool,
    func: impl FreeFnExt + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<FreePrimitiveExt>::new(name, func);
    let func = Func::new_primitive(input_mode, output_mode, cacheable, primitive);
    let func_val = FreeFuncVal::from(func);
    Named::new(name, FuncVal::Free(func_val))
}

fn named_static_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    cacheable: bool,
    func: impl StaticFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<StaticPrimitiveExt>::new(name, func);
    let func = Func::new_primitive(input_mode, output_mode, cacheable, primitive);
    let func_val = StaticFuncVal::from(func);
    Named::new(name, FuncVal::Static(func_val))
}

fn named_const_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    cacheable: bool,
    func: impl ConstFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<ConstPrimitiveExt>::new(name, func);
    let func = Func::new_primitive(input_mode, output_mode, cacheable, primitive);
    let func_val = ConstFuncVal::from(func);
    Named::new(name, FuncVal::Const(func_val))
}

fn named_mut_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    cacheable: bool,
    func: impl MutFn + 'static,
) -> Named<FuncVal> {
    let primitive = Primitive::<MutPrimitiveExt>::new(name, func);
    let func = Func::new_primitive(input_mode, output_mode, cacheable, primitive);
    let func_val = MutFuncVal::from(func);
    Named::new(name, FuncVal::Mut(func_val))
}

fn id_mode() -> Mode {
    Mode {
        default: BasicMode::Id,
        specialized: None,
    }
}

fn form_mode() -> Mode {
    Mode {
        default: BasicMode::Form,
        specialized: None,
    }
}

#[allow(unused)]
fn eval_mode() -> Mode {
    Mode {
        default: BasicMode::Eval,
        specialized: None,
    }
}

fn pair_mode(first: Mode, second: Mode, default: BasicMode) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        pair: Pair::new(first, second),
        list: ListMode {
            head: List::default(),
            tail: default_mode.clone(),
        },
        map: MapMode {
            some: Map::default(),
            else1: Pair::new(default_mode.clone(), default_mode),
        },
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

#[allow(unused)]
fn list_mode(head: List<Mode>, tail: Mode, default: BasicMode) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        list: ListMode { head, tail },
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        map: MapMode {
            some: Map::default(),
            else1: Pair::new(default_mode.clone(), default_mode),
        },
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

fn map_mode(some: Map<Val, Mode>, key: Mode, value: Mode, default: BasicMode) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let else1 = Pair::new(key, value);
    let val_mode = ValMode {
        map: MapMode { some, else1 },
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        list: ListMode {
            head: List::default(),
            tail: default_mode,
        },
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

mod mode;

mod func;

mod call;

mod ask;

mod case;

mod answer;

mod symbol;

mod unit;

mod bool;

mod int;

mod number;

mod byte;

mod text;

mod pair;

mod list;

mod map;

mod extension;

mod comment;
