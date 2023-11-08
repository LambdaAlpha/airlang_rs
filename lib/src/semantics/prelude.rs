use crate::{
    semantics::{
        ctx::{
            Ctx,
            NameMap,
            TaggedVal,
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
        val::Val,
        Func,
    },
    types::{
        Reader,
        Symbol,
    },
};

thread_local! (static PRELUDE: NameMap = init_prelude());

pub(crate) fn prelude() -> NameMap {
    PRELUDE.with(Clone::clone)
}

pub(crate) fn initial_ctx() -> Ctx {
    Ctx {
        name_map: prelude(),
        super_ctx: None,
    }
}

fn init_prelude() -> NameMap {
    let mut c = NameMap::default();

    prelude_meta(&mut c);
    prelude_syntax(&mut c);
    prelude_value(&mut c);
    prelude_ctx(&mut c);
    prelude_ctrl(&mut c);
    prelude_eval(&mut c);
    prelude_logic(&mut c);
    prelude_func(&mut c);
    prelude_call(&mut c);
    prelude_prop(&mut c);
    prelude_bool(&mut c);
    prelude_int(&mut c);
    prelude_str(&mut c);
    prelude_pair(&mut c);
    prelude_list(&mut c);
    prelude_map(&mut c);

    c
}

fn prelude_meta(c: &mut NameMap) {
    put(c, names::AIR_VERSION_MAJOR, meta::version_major());
    put(c, names::AIR_VERSION_MINOR, meta::version_minor());
    put(c, names::AIR_VERSION_PATCH, meta::version_patch());
}

fn prelude_syntax(c: &mut NameMap) {
    put_primitive_func(c, syntax::parse());
    put_primitive_func(c, syntax::stringify());
}

fn prelude_value(c: &mut NameMap) {
    put_primitive_func(c, value::type_of());
    put_primitive_func(c, value::equal());
    put_primitive_func(c, value::not_equal());
}

fn prelude_ctx(c: &mut NameMap) {
    put_primitive_func(c, ctx::read());
    put_primitive_func(c, ctx::remove());
    put_primitive_func(c, ctx::assign());
    put_primitive_func(c, ctx::set_final());
    put_primitive_func(c, ctx::set_const());
    put_primitive_func(c, ctx::is_final());
    put_primitive_func(c, ctx::is_const());
    put_primitive_func(c, ctx::is_null());
    put_primitive_func(c, ctx::is_local());
    put_primitive_func(c, ctx::ctx_get_super());
    put_primitive_func(c, ctx::ctx_set_super());
    put_primitive_func(c, ctx::ctx_new());
    put_primitive_func(c, ctx::ctx_repr());
    put_primitive_func(c, ctx::ctx_prelude());
    put_primitive_func(c, ctx::ctx_current());
}

fn prelude_ctrl(c: &mut NameMap) {
    put_primitive_func(c, ctrl::sequence());
    put_primitive_func(c, ctrl::breakable_sequence());
    put_primitive_func(c, ctrl::condition());
    put_primitive_func(c, ctrl::matching());
    put_primitive_func(c, ctrl::while_loop());
}

fn prelude_eval(c: &mut NameMap) {
    put_primitive_func(c, eval::value());
    put_primitive_func(c, eval::eval());
    put_primitive_func(c, eval::quote());
    put_primitive_func(c, eval::eval_twice());
    put_primitive_func(c, eval::eval_thrice());
}

fn prelude_logic(c: &mut NameMap) {
    put_primitive_func(c, logic::theorem_new());
    put_primitive_func(c, logic::prove());
}

fn prelude_func(c: &mut NameMap) {
    put_primitive_func(c, func::func_new());
    put_primitive_func(c, func::func_repr());
    put_primitive_func(c, func::func_input_mode());
    put_primitive_func(c, func::func_access());
    put_primitive_func(c, func::func_is_primitive());
    put_primitive_func(c, func::func_id());
    put_primitive_func(c, func::func_body());
    put_primitive_func(c, func::func_context());
    put_primitive_func(c, func::func_input_name());
    put_primitive_func(c, func::func_caller_name());
}

fn prelude_call(c: &mut NameMap) {
    put_primitive_func(c, call::chain());
    put_primitive_func(c, call::call_with_ctx());
}

fn prelude_prop(c: &mut NameMap) {
    put_primitive_func(c, prop::prop_new());
    put_primitive_func(c, prop::prop_repr());
    put_primitive_func(c, prop::get_truth());
    put_primitive_func(c, prop::get_function());
    put_primitive_func(c, prop::get_input());
    put_primitive_func(c, prop::get_output());
    put_primitive_func(c, prop::get_before());
    put_primitive_func(c, prop::get_after());
}

fn prelude_bool(c: &mut NameMap) {
    put_primitive_func(c, bool::not());
    put_primitive_func(c, bool::and());
    put_primitive_func(c, bool::or());
}

fn prelude_int(c: &mut NameMap) {
    put_primitive_func(c, int::add());
    put_primitive_func(c, int::subtract());
    put_primitive_func(c, int::multiply());
    put_primitive_func(c, int::divide());
    put_primitive_func(c, int::remainder());
    put_primitive_func(c, int::divide_remainder());
    put_primitive_func(c, int::less_than());
    put_primitive_func(c, int::less_equal());
    put_primitive_func(c, int::greater_than());
    put_primitive_func(c, int::greater_equal());
    put_primitive_func(c, int::less_greater());
}

fn prelude_str(c: &mut NameMap) {
    put_primitive_func(c, str::length());
    put_primitive_func(c, str::concat());
}

fn prelude_pair(c: &mut NameMap) {
    put_primitive_func(c, pair::first());
    put_primitive_func(c, pair::first_assign());
    put_primitive_func(c, pair::second());
    put_primitive_func(c, pair::second_assign());
}

fn prelude_list(c: &mut NameMap) {
    put_primitive_func(c, list::length());
    put_primitive_func(c, list::set());
    put_primitive_func(c, list::set_many());
    put_primitive_func(c, list::get());
    put_primitive_func(c, list::insert());
    put_primitive_func(c, list::insert_many());
    put_primitive_func(c, list::remove());
    put_primitive_func(c, list::push());
    put_primitive_func(c, list::push_many());
    put_primitive_func(c, list::pop());
    put_primitive_func(c, list::clear());
}

fn prelude_map(c: &mut NameMap) {
    put_primitive_func(c, map::length());
    put_primitive_func(c, map::keys());
    put_primitive_func(c, map::into_keys());
    put_primitive_func(c, map::values());
    put_primitive_func(c, map::into_values());
    put_primitive_func(c, map::contains());
    put_primitive_func(c, map::contains_many());
    put_primitive_func(c, map::set());
    put_primitive_func(c, map::set_many());
    put_primitive_func(c, map::get());
    put_primitive_func(c, map::get_many());
    put_primitive_func(c, map::remove());
    put_primitive_func(c, map::remove_many());
    put_primitive_func(c, map::clear());
}

fn put(c: &mut NameMap, key: &str, val: Val) {
    c.insert(Symbol::from_str(key), TaggedVal::new_const(val));
}

fn put_primitive_func<F>(c: &mut NameMap, primitive: PrimitiveFunc<F>)
where
    PrimitiveFunc<F>: Into<Func>,
{
    let id = primitive.evaluator.get_id().clone();
    let func = primitive.into();
    let val = Val::Func(Reader::new(func).into());
    c.insert(id, TaggedVal::new_const(val));
}

pub(crate) struct PrimitiveFunc<F> {
    input_mode: InputMode,
    evaluator: Primitive<F>,
}

#[allow(clippy::from_over_into)]
impl Into<Func> for PrimitiveFunc<CtxFreeFn> {
    fn into(self) -> Func {
        let evaluator = FuncEval::Free(FuncImpl::Primitive(self.evaluator));
        Func::new(self.input_mode, evaluator)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Func> for PrimitiveFunc<CtxConstFn> {
    fn into(self) -> Func {
        let evaluator = FuncEval::Const(FuncImpl::Primitive(self.evaluator));
        Func::new(self.input_mode, evaluator)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Func> for PrimitiveFunc<CtxMutableFn> {
    fn into(self) -> Func {
        let evaluator = FuncEval::Mutable(FuncImpl::Primitive(self.evaluator));
        Func::new(self.input_mode, evaluator)
    }
}

impl<F> PrimitiveFunc<F> {
    fn new(input_mode: InputMode, primitive: Primitive<F>) -> Self {
        PrimitiveFunc {
            input_mode,
            evaluator: primitive,
        }
    }
}

pub(crate) mod names;

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
