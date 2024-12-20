use crate::{
    FuncMode,
    bit::Bit,
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
    },
    func::{
        FuncTrait,
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
        named_free_fn,
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
    pub(crate) abstract_mode: Named<FuncVal>,
    pub(crate) ask_mode: Named<FuncVal>,
    pub(crate) is_cacheable: Named<FuncVal>,
    pub(crate) is_primitive: Named<FuncVal>,
    pub(crate) is_extension: Named<FuncVal>,
    pub(crate) is_cell: Named<FuncVal>,
    pub(crate) is_mode: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) body_mode: Named<FuncVal>,
    pub(crate) body: Named<FuncVal>,
    pub(crate) ctx: Named<FuncVal>,
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
            abstract_mode: abstract_mode(),
            ask_mode: ask_mode(),
            is_cacheable: is_cacheable(),
            is_primitive: is_primitive(),
            is_extension: is_extension(),
            is_cell: is_cell(),
            is_mode: is_mode(),
            id: id(),
            body_mode: body_mode(),
            body: body(),
            ctx: ctx(),
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
        self.abstract_mode.put(m);
        self.ask_mode.put(m);
        self.is_cacheable.put(m);
        self.is_primitive.put(m);
        self.is_extension.put(m);
        self.is_cell.put(m);
        self.is_mode.put(m);
        self.id.put(m);
        self.body_mode.put(m);
        self.body.put(m);
        self.ctx.put(m);
        self.input_name.put(m);
        self.ctx_name.put(m);
    }
}

fn mode_id() -> Named<FuncVal> {
    let id = "id";
    let func = ModeFunc::new(Mode::Primitive(PrimitiveMode::Id));
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_form() -> Named<FuncVal> {
    let id = "form";
    let func = ModeFunc::new(Mode::Primitive(PrimitiveMode::Form));
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode_eval() -> Named<FuncVal> {
    let id = "eval";
    let func = ModeFunc::new(Mode::Primitive(PrimitiveMode::Eval));
    let f = FuncVal::Mode(func.into());
    Named::new(id, f)
}

fn mode() -> Named<FuncVal> {
    let id = "mode";
    let f = fn_mode;
    let call = form_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_mode(input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = ModeFunc::new(mode);
    Val::Func(FuncVal::Mode(func.into()))
}

fn new() -> Named<FuncVal> {
    let id = "function";
    let f = fn_new;
    let call = parse_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_new(input: Val) -> Val {
    match parse_func(input) {
        Some(func) => Val::Func(func),
        None => Val::default(),
    }
}

fn repr() -> Named<FuncVal> {
    let id = "function.represent";
    let f = fn_repr;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = generate_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        return Val::default();
    };
    generate_func(func)
}

fn ctx_access() -> Named<FuncVal> {
    let id = "function.context_access";
    let f = fn_ctx_access;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_ctx_access(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let access = match func {
            FuncVal::Mode(_) => MUTABLE,
            FuncVal::FreeCellPrim(_) => FREE,
            FuncVal::FreeCellComp(_) => FREE,
            FuncVal::FreeStaticPrim(_) => FREE,
            FuncVal::FreeStaticComp(_) => FREE,
            FuncVal::ConstStaticPrim(_) => CONST,
            FuncVal::ConstStaticComp(_) => CONST,
            FuncVal::MutStaticPrim(_) => MUTABLE,
            FuncVal::MutStaticComp(_) => MUTABLE,
        };
        Val::Symbol(Symbol::from_str(access))
    })
}

fn call_mode() -> Named<FuncVal> {
    let id = "function.call_mode";
    let f = fn_call_mode;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_call_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let mode = func.mode().call.clone();
        Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
    })
}

fn abstract_mode() -> Named<FuncVal> {
    let id = "function.abstract_mode";
    let f = fn_abstract_mode;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_abstract_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let mode = func.mode().abstract1.clone();
        Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
    })
}

fn ask_mode() -> Named<FuncVal> {
    let id = "function.ask_mode";
    let f = fn_ask_mode;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_ask_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let mode = func.mode().ask.clone();
        Val::Func(FuncVal::Mode(ModeFunc::new(mode).into()))
    })
}

fn is_cacheable() -> Named<FuncVal> {
    let id = "function.is_cacheable";
    let f = fn_is_cacheable;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_cacheable(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let cacheable = func.cacheable();
        Val::Bit(Bit::new(cacheable))
    })
}

fn is_primitive() -> Named<FuncVal> {
    let id = "function.is_primitive";
    let f = fn_is_primitive;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_primitive(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let is_primitive = func.is_primitive();
        Val::Bit(Bit::new(is_primitive))
    })
}

fn is_extension() -> Named<FuncVal> {
    let id = "function.is_extension";
    let f = fn_is_extension;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_extension(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(primitive) = func.primitive() else {
            return Val::default();
        };
        Val::Bit(Bit::new(primitive.is_extension))
    })
}

fn is_cell() -> Named<FuncVal> {
    let id = "function.is_cell";
    let f = fn_is_cell;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_cell(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let is_cell = matches!(func, FuncVal::FreeCellPrim(_) | FuncVal::FreeCellComp(_));
        Val::Bit(Bit::new(is_cell))
    })
}

fn is_mode() -> Named<FuncVal> {
    let id = "function.is_mode";
    let f = fn_is_mode;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        Val::Bit(Bit::new(matches!(func, FuncVal::Mode(_))))
    })
}

fn id() -> Named<FuncVal> {
    let id = "function.id";
    let f = fn_id;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_id(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(primitive) = func.primitive() else {
            return Val::default();
        };
        Val::Symbol(primitive.id.clone())
    })
}

fn body_mode() -> Named<FuncVal> {
    let id = "function.body_mode";
    let f = fn_body_mode;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_body_mode(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(composite) = func.composite() else {
            return Val::default();
        };
        Val::Func(FuncVal::Mode(
            ModeFunc::new(composite.body_mode.clone()).into(),
        ))
    })
}

fn body() -> Named<FuncVal> {
    let id = "function.body";
    let f = fn_body;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = form_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_body(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(composite) = func.composite() else {
            return Val::default();
        };
        composite.body.clone()
    })
}

fn ctx() -> Named<FuncVal> {
    let id = "function.context";
    let f = fn_ctx;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_ctx(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(composite) = func.composite() else {
            return Val::default();
        };
        Val::Ctx(CtxVal::from(composite.ctx.clone()))
    })
}

fn input_name() -> Named<FuncVal> {
    let id = "function.input_name";
    let f = fn_input_name;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_input_name(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        let Some(composite) = func.composite() else {
            return Val::default();
        };
        Val::Symbol(composite.input_name.clone())
    })
}

fn ctx_name() -> Named<FuncVal> {
    let id = "function.context_name";
    let f = fn_ctx_name;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_ctx_name(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Func(func) = val else {
            return Val::default();
        };
        match func {
            FuncVal::ConstStaticComp(f) => Val::Symbol(f.ctx_name.clone()),
            FuncVal::MutStaticComp(f) => Val::Symbol(f.ctx_name.clone()),
            _ => Val::default(),
        }
    })
}
