use crate::{
    bool::Bool,
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
    },
    func::{
        mode::ModeFunc,
        repr::{
            CONST,
            FREE,
            MUTABLE,
            generate_func,
            generate_mode,
            parse_func,
            parse_mode,
        },
    },
    map::Map,
    mode::{
        Mode,
        primitive::PrimitiveMode,
        repr::parse,
    },
    prelude::{
        Named,
        Prelude,
        form_mode,
        named_const_fn,
        named_static_fn,
    },
    symbol::Symbol,
    val::{
        Val,
        ctx::CtxVal,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct FuncPrelude {
    pub(crate) mode_id: Named<FuncVal>,
    pub(crate) mode_form: Named<FuncVal>,
    pub(crate) mode_eval: Named<FuncVal>,
    pub(crate) mode: Named<FuncVal>,
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) ctx_access: Named<FuncVal>,
    pub(crate) call_mode: Named<FuncVal>,
    pub(crate) ask_mode: Named<FuncVal>,
    pub(crate) is_cacheable: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) is_static: Named<FuncVal>,
    pub(crate) is_mode: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) body_mode: Named<FuncVal>,
    pub(crate) body: Named<FuncVal>,
    pub(crate) prelude: Named<FuncVal>,
    pub(crate) input_name: Named<FuncVal>,
    pub(crate) ctx_name: Named<FuncVal>,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            mode_id: mode_id(),
            mode_form: mode_form(),
            mode_eval: mode_eval(),
            mode: mode(),
            new: new(),
            repr: repr(),
            ctx_access: ctx_access(),
            call_mode: call_mode(),
            ask_mode: ask_mode(),
            is_cacheable: is_cacheable(),
            is_primitive: is_primitive(),
            is_extension: is_extension(),
            is_static: is_static(),
            is_mode: is_mode(),
            id: id(),
            body_mode: body_mode(),
            body: body(),
            prelude: prelude(),
            input_name: input_name(),
            ctx_name: ctx_name(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.mode_id.put(m);
        self.mode_form.put(m);
        self.mode_eval.put(m);
        self.mode.put(m);
        self.new.put(m);
        self.repr.put(m);
        self.ctx_access.put(m);
        self.call_mode.put(m);
        self.ask_mode.put(m);
        self.is_cacheable.put(m);
        self.is_primitive.put(m);
        self.is_extension.put(m);
        self.is_static.put(m);
        self.is_mode.put(m);
        self.id.put(m);
        self.body_mode.put(m);
        self.body.put(m);
        self.prelude.put(m);
        self.input_name.put(m);
        self.ctx_name.put(m);
    }
}

fn mode_id() -> Named<FuncVal> {
    let func = ModeFunc::new(Mode::Primitive(PrimitiveMode::Id));
    let func = FuncVal::Mode(func.into());
    Named::new("id", func)
}

fn mode_form() -> Named<FuncVal> {
    let func = ModeFunc::new(Mode::Primitive(PrimitiveMode::Form));
    let func = FuncVal::Mode(func.into());
    Named::new("form", func)
}

fn mode_eval() -> Named<FuncVal> {
    let func = ModeFunc::new(Mode::Primitive(PrimitiveMode::Eval));
    let func = FuncVal::Mode(func.into());
    Named::new("eval", func)
}

fn mode() -> Named<FuncVal> {
    let call_mode = form_mode();
    let ask_mode = Mode::default();
    named_static_fn("mode", call_mode, ask_mode, true, fn_mode)
}

fn fn_mode(input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = ModeFunc::new(mode);
    Val::Func(FuncVal::Mode(func.into()))
}

fn new() -> Named<FuncVal> {
    let call_mode = parse_mode();
    let ask_mode = Mode::default();
    named_static_fn("function", call_mode, ask_mode, true, fn_new)
}

fn fn_new(input: Val) -> Val {
    match parse_func(input) {
        Some(func) => Val::Func(func),
        None => Val::default(),
    }
}

fn repr() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = generate_mode();
    named_static_fn("function.represent", call_mode, ask_mode, true, fn_repr)
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    generate_func(func)
}

fn ctx_access() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.context_access",
        call_mode,
        ask_mode,
        true,
        fn_ctx_access,
    )
}

fn fn_ctx_access(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let access = match func {
            FuncVal::Mode(_) => MUTABLE,
            FuncVal::Free(_) => FREE,
            FuncVal::Static(_) => FREE,
            FuncVal::Const(_) => CONST,
            FuncVal::Mut(_) => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

fn call_mode() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.call_mode",
        call_mode,
        ask_mode,
        true,
        fn_call_mode,
    )
}

fn fn_call_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let mode = func.call_mode().clone();
        Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
    })
}

fn ask_mode() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.ask_mode", call_mode, ask_mode, true, fn_ask_mode)
}

fn fn_ask_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let mode = func.ask_mode().clone();
        Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
    })
}

fn is_cacheable() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_cacheable",
        call_mode,
        ask_mode,
        true,
        fn_is_cacheable,
    )
}

fn fn_is_cacheable(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let cacheable = func.cacheable();
        Val::Bool(Bool::new(cacheable))
    })
}

fn is_primitive() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_primitive",
        call_mode,
        ask_mode,
        true,
        fn_is_primitive,
    )
}

fn fn_is_primitive(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bool(Bool::new(is_primitive))
    })
}

fn is_extension() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_extension",
        call_mode,
        ask_mode,
        true,
        fn_is_extension,
    )
}

fn fn_is_extension(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(is_extension) = func.is_extension() else {
            return Val::default();
        };
        Val::Bool(Bool::new(is_extension))
    })
}

fn is_static() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.is_static",
        call_mode,
        ask_mode,
        true,
        fn_is_static,
    )
}

fn fn_is_static(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        Val::Bool(Bool::new(matches!(func, FuncVal::Static(_))))
    })
}

fn is_mode() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.is_mode", call_mode, ask_mode, true, fn_is_mode)
}

fn fn_is_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        Val::Bool(Bool::new(matches!(func, FuncVal::Mode(_))))
    })
}

fn id() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.id", call_mode, ask_mode, true, fn_id)
}

fn fn_id(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(id) = func.id() else {
            return Val::default();
        };
        Val::Symbol(id)
    })
}

fn body_mode() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.body_mode",
        call_mode,
        ask_mode,
        true,
        fn_body_mode,
    )
}

fn fn_body_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(mode) = func.body_mode() else {
            return Val::default();
        };
        Val::Func(FuncVal::Mode(ModeFunc::new(mode.clone()).into()))
    })
}

fn body() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = form_mode();
    named_const_fn("function.body", call_mode, ask_mode, true, fn_body)
}

fn fn_body(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(body) = func.body() else {
            return Val::default();
        };
        body.clone()
    })
}

fn prelude() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("function.prelude", call_mode, ask_mode, true, fn_prelude)
}

fn fn_prelude(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(prelude) = func.prelude() else {
            return Val::default();
        };
        Val::Ctx(CtxVal::from(prelude.clone()))
    })
}

fn input_name() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.input_name",
        call_mode,
        ask_mode,
        true,
        fn_input_name,
    )
}

fn fn_input_name(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(name) = func.input_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}

fn ctx_name() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "function.context_name",
        call_mode,
        ask_mode,
        true,
        fn_ctx_name,
    )
}

fn fn_ctx_name(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(name) = func.ctx_name() else {
            return Val::default();
        };
        Val::Symbol(name)
    })
}
