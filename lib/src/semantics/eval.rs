use {
    crate::{
        semantics::val::{
            CtxVal,
            FuncVal,
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
        types::{
            Call,
            Either,
            Keeper,
            Map,
            Owner,
            Pair,
            Reader,
            Reverse,
        },
    },
    smartstring::alias::CompactString,
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::{
            Hash,
            Hasher,
        },
        mem::swap,
        ops::Deref,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Func {
    pub(crate) func_trait: FuncTrait,
    pub(crate) func_impl: FuncImpl,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct FuncTrait {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum FuncImpl {
    Primitive(Primitive),
    Composed(Composed),
}

#[derive(Debug, Clone)]
pub(crate) struct Primitive {
    pub(crate) id: Name,
    pub(crate) eval: PrimitiveEval,
}

#[derive(Clone)]
pub(crate) enum PrimitiveEval {
    CtxFree {
        eval_mode: EvalMode,
        evaluator: CtxFreeFn,
    },
    CtxAware {
        evaluator: CtxAwareFn,
    },
}

type CtxFreeFn = Reader<dyn Fn(Val) -> Val>;

type CtxAwareFn = Reader<dyn Fn(&mut Ctx, Val) -> Val>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Name,
    pub(crate) eval: ComposedEval,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum ComposedEval {
    CtxFree { eval_mode: EvalMode },
    CtxAware { caller_name: Name },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EvalMode {
    Value,
    Eval,
    Interpolate,
    Inline,
}

pub(crate) type Name = CompactString;

pub(crate) type NameMap = Map<Name, TaggedVal>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InvariantTag {
    None,
    // can't be assigned
    Final,
    // can't be modified
    Const,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct TaggedVal {
    pub(crate) tag: InvariantTag,
    pub(crate) val: Val,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Ctx {
    pub(crate) name_map: NameMap,
    pub(crate) super_ctx: Option<Either<Name, RefVal>>,
    pub(crate) reverse_interpreter: Option<Reader<Func>>,
}

impl Func {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.func_impl.eval(ctx, input)
    }
}

impl FuncImpl {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval(ctx, input),
            FuncImpl::Composed(c) => c.eval(ctx, input),
        }
    }
}

impl Primitive {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match &self.eval {
            PrimitiveEval::CtxFree {
                eval_mode,
                evaluator,
            } => {
                let val = eval_mode.eval(ctx, input);
                evaluator(val)
            }
            PrimitiveEval::CtxAware { evaluator } => evaluator(ctx, input),
        }
    }
}

impl Composed {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        let mut new_ctx = self.ctx.clone();
        new_ctx.reverse_interpreter = ctx.reverse_interpreter.clone();

        match &self.eval {
            ComposedEval::CtxFree { eval_mode } => {
                let val = eval_mode.eval(ctx, input);
                new_ctx.put_val_local(self.input_name.clone(), TaggedVal::new(val));
                new_ctx.eval_by_ref(&self.body)
            }
            ComposedEval::CtxAware { caller_name } => {
                new_ctx.put_val_local(self.input_name.clone(), TaggedVal::new(input));

                let mut caller_ctx = Ctx::default();
                swap(ctx, &mut caller_ctx);
                let keeper =
                    Keeper::new(TaggedVal::new_final(Val::Ctx(Box::new(caller_ctx).into())));
                let caller_ctx_ref = Val::Ref(RefVal(keeper.clone()));
                new_ctx.put_val_local(caller_name.clone(), TaggedVal::new(caller_ctx_ref));

                let output = new_ctx.eval_by_ref(&self.body);

                if let Ok(o) = Keeper::owner(&keeper) {
                    if let Val::Ctx(CtxVal(caller)) = Owner::move_data(o).val {
                        *ctx = *caller;
                    }
                }

                output
            }
        }
    }
}

impl EvalMode {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Value => input,
            EvalMode::Eval => ctx.eval(input),
            EvalMode::Interpolate => ctx.eval_interpolate(input),
            EvalMode::Inline => ctx.eval_inline(input),
        }
    }
}

impl Ctx {
    pub(crate) fn eval(&mut self, input: Val) -> Val {
        match input {
            Val::Symbol(s) => self.get(&s),
            Val::Ref(k) => self.eval_ref(&k),
            Val::Pair(p) => self.eval_pair(p.first, p.second),
            Val::List(l) => self.eval_list(l),
            Val::Map(m) => self.eval_map(m),
            Val::Call(c) => self.eval_call(c.func, c.input),
            Val::Reverse(r) => self.eval_reverse(r.func, r.output),
            v => v,
        }
    }

    pub(crate) fn eval_ref(&self, ref_val: &RefVal) -> Val {
        let Ok(input) = Keeper::reader(&ref_val.0) else {
            return Val::default();
        };
        input.deref().val.clone()
    }

    pub(crate) fn eval_pair(&mut self, first: Val, second: Val) -> Val {
        let pair = Pair::new(self.eval(first), self.eval(second));
        Val::Pair(Box::new(pair))
    }

    pub(crate) fn eval_list(&mut self, list: ListVal) -> Val {
        let list = list.into_iter().map(|v| self.eval(v)).collect();
        Val::List(list)
    }

    pub(crate) fn eval_map(&mut self, map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| (self.eval_inline(k), self.eval(v)))
            .collect();
        Val::Map(map)
    }

    pub(crate) fn eval_call(&mut self, func: Val, input: Val) -> Val {
        let Val::Func(FuncVal(func)) = self.eval(func) else {
            return Val::default();
        };
        func.eval(self, input)
    }

    pub(crate) fn eval_reverse(&mut self, func: Val, output: Val) -> Val {
        let reverse_interpreter = self.reverse_interpreter.clone();
        let Some(reverse_interpreter) = reverse_interpreter else {
            return Val::default();
        };
        let reverse_func = reverse_interpreter.deref().clone().eval(self, func);
        self.eval_call(reverse_func, output)
    }

    pub(crate) fn eval_by_ref(&mut self, input: &Val) -> Val {
        match input {
            Val::Symbol(s) => self.get(s),
            Val::Ref(k) => self.eval_ref(k),
            Val::Pair(p) => self.eval_pair_by_ref(&p.first, &p.second),
            Val::List(l) => self.eval_list_by_ref(l),
            Val::Map(m) => self.eval_map_by_ref(m),
            Val::Call(c) => self.eval_call_by_ref(&c.func, &c.input),
            Val::Reverse(r) => self.eval_reverse_by_ref(&r.func, &r.output),
            v => v.clone(),
        }
    }

    pub(crate) fn eval_pair_by_ref(&mut self, first: &Val, second: &Val) -> Val {
        let pair = Pair::new(self.eval_by_ref(first), self.eval_by_ref(second));
        Val::Pair(Box::new(pair))
    }

    pub(crate) fn eval_list_by_ref(&mut self, list: &ListVal) -> Val {
        let list = list.into_iter().map(|v| self.eval_by_ref(v)).collect();
        Val::List(list)
    }

    pub(crate) fn eval_map_by_ref(&mut self, map: &MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| (self.eval_inline_by_ref(k), self.eval_by_ref(v)))
            .collect();
        Val::Map(map)
    }

    pub(crate) fn eval_call_by_ref(&mut self, func: &Val, input: &Val) -> Val {
        let Val::Func(FuncVal(func)) = self.eval_by_ref(func) else {
            return Val::default();
        };
        func.eval(self, input.clone())
    }

    pub(crate) fn eval_reverse_by_ref(&mut self, func: &Val, output: &Val) -> Val {
        self.eval_reverse(func.clone(), output.clone())
    }

    pub(crate) fn eval_interpolate(&mut self, input: Val) -> Val {
        match input {
            Val::Call(c) => match &c.func {
                Val::Unit(_) => self.eval(c.input),
                _ => {
                    let func = self.eval_interpolate(c.func);
                    let input = self.eval_interpolate(c.input);
                    let call = Box::new(Call::new(func, input));
                    Val::Call(call)
                }
            },
            Val::Pair(p) => {
                let first = self.eval_interpolate(p.first);
                let second = self.eval_interpolate(p.second);
                let pair = Box::new(Pair::new(first, second));
                Val::Pair(pair)
            }
            Val::Reverse(r) => {
                let func = self.eval_interpolate(r.func);
                let output = self.eval_interpolate(r.output);
                let reverse = Box::new(Reverse::new(func, output));
                Val::Reverse(reverse)
            }
            Val::List(l) => {
                let list = l.into_iter().map(|v| self.eval_interpolate(v)).collect();
                Val::List(list)
            }
            Val::Map(m) => {
                let map = m
                    .into_iter()
                    .map(|(k, v)| {
                        let key = self.eval_interpolate(k);
                        let value = self.eval_interpolate(v);
                        (key, value)
                    })
                    .collect();
                Val::Map(map)
            }
            i => i,
        }
    }

    #[allow(unused)]
    pub(crate) fn eval_interpolate_by_ref(&mut self, input: &Val) -> Val {
        match input {
            Val::Call(c) => match &c.func {
                Val::Unit(_) => self.eval_by_ref(&c.input),
                _ => {
                    let func = self.eval_interpolate_by_ref(&c.func);
                    let input = self.eval_interpolate_by_ref(&c.input);
                    let call = Box::new(Call::new(func, input));
                    Val::Call(call)
                }
            },
            Val::Pair(p) => {
                let first = self.eval_interpolate_by_ref(&p.first);
                let second = self.eval_interpolate_by_ref(&p.second);
                let pair = Box::new(Pair::new(first, second));
                Val::Pair(pair)
            }
            Val::Reverse(r) => {
                let func = self.eval_interpolate_by_ref(&r.func);
                let output = self.eval_interpolate_by_ref(&r.output);
                let reverse = Box::new(Reverse::new(func, output));
                Val::Reverse(reverse)
            }
            Val::List(l) => {
                let list = l
                    .into_iter()
                    .map(|v| self.eval_interpolate_by_ref(v))
                    .collect();
                Val::List(list)
            }
            Val::Map(m) => {
                let map = m
                    .into_iter()
                    .map(|(k, v)| {
                        let key = self.eval_interpolate_by_ref(k);
                        let value = self.eval_interpolate_by_ref(v);
                        (key, value)
                    })
                    .collect();
                Val::Map(map)
            }
            i => i.clone(),
        }
    }

    pub(crate) fn eval_inline(&mut self, input: Val) -> Val {
        match input {
            Val::Call(call) => self.eval_call(call.func, call.input),
            Val::Reverse(reverse) => self.eval_reverse(reverse.func, reverse.output),
            Val::Pair(pair) => {
                let first = self.eval_inline(pair.first);
                let second = self.eval_inline(pair.second);
                let pair = Box::new(Pair::new(first, second));
                Val::Pair(pair)
            }
            Val::List(l) => {
                let list = l.into_iter().map(|v| self.eval_inline(v)).collect();
                Val::List(list)
            }
            Val::Map(m) => {
                let map = m
                    .into_iter()
                    .map(|(k, v)| {
                        let key = self.eval_inline(k);
                        let value = self.eval_inline(v);
                        (key, value)
                    })
                    .collect();
                Val::Map(map)
            }
            v => v,
        }
    }

    pub(crate) fn eval_inline_by_ref(&mut self, input: &Val) -> Val {
        match input {
            Val::Call(call) => self.eval_call_by_ref(&call.func, &call.input),
            Val::Reverse(reverse) => self.eval_reverse_by_ref(&reverse.func, &reverse.output),
            Val::Pair(pair) => {
                let first = self.eval_inline_by_ref(&pair.first);
                let second = self.eval_inline_by_ref(&pair.second);
                let pair = Box::new(Pair::new(first, second));
                Val::Pair(pair)
            }
            Val::List(l) => {
                let list = l.into_iter().map(|v| self.eval_inline_by_ref(v)).collect();
                Val::List(list)
            }
            Val::Map(m) => {
                let map = m
                    .into_iter()
                    .map(|(k, v)| {
                        let key = self.eval_inline_by_ref(k);
                        let value = self.eval_inline_by_ref(v);
                        (key, value)
                    })
                    .collect();
                Val::Map(map)
            }
            v => v.clone(),
        }
    }

    pub(crate) fn get(&self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return Val::default();
                };
                ctx.get(name)
            });
        };
        tagged_val.val.clone()
    }

    pub(crate) fn get_ref<T, F>(&self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<&Val>) -> T,
    {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return f(None);
                };
                ctx.get_ref(name, f)
            });
        };
        f(Some(&tagged_val.val))
    }

    pub(crate) fn get_mut<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<&mut Val>) -> T,
    {
        if self.name_map.get(name).is_none() {
            return self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.get_mut(name, f)
                } else {
                    f(None)
                }
            });
        }
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            return f(None);
        };

        if matches!(tagged_val.tag, InvariantTag::Const) {
            return f(None);
        }

        f(Some(&mut tagged_val.val))
    }

    pub(crate) fn remove(&mut self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.remove(name)
                } else {
                    Val::default()
                }
            });
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn put_val(&mut self, name: Name, val: TaggedVal) -> Val {
        let Some(tagged_val) = self.name_map.get(&name) else {
            return self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.put_val(name, val)
                } else {
                    ctx.put_unchecked(name, val)
                }
            });
        };

        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.put_unchecked(name, val)
    }

    pub(crate) fn put_val_local(&mut self, name: Name, val: TaggedVal) -> Val {
        let (None | Some(TaggedVal { tag: InvariantTag::None, .. })) = self.name_map.get(&name) else {
            return Val::default();
        };
        self.put_unchecked(name, val)
    }

    fn put_unchecked(&mut self, name: Name, val: TaggedVal) -> Val {
        self.name_map
            .insert(name, val)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn set_final(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.set_final(name);
                }
            });
            return;
        };
        if !(matches!(&tagged_val.tag, InvariantTag::None)) {
            return;
        }
        tagged_val.tag = InvariantTag::Final;
    }

    pub(crate) fn set_const(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.set_const(name);
                }
            });
            return;
        };
        tagged_val.tag = InvariantTag::Const;
    }

    pub(crate) fn is_final(&self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return false;
                };
                ctx.is_final(name)
            });
        };
        matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const)
    }

    pub(crate) fn is_const(&self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return false;
                };
                ctx.is_const(name)
            });
        };
        matches!(&tagged_val.tag, InvariantTag::Const)
    }

    fn get_ref_super_ctx<T, F>(&self, f: F) -> T
    where
        F: FnOnce(Option<&Ctx>) -> T,
    {
        let Some(name_or_ref) = &self.super_ctx else {
            return f(None);
        };
        match name_or_ref {
            Either::Left(name) => {
                let Some(TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), .. }) = self.name_map.get(name) else {
                    return f(None);
                };
                f(Some(super_ctx))
            }
            Either::Right(RefVal(r)) => {
                let Ok(r) = Keeper::reader(r) else {
                    return f(None);
                };
                let TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), .. } = &*r else {
                    return f(None);
                };
                f(Some(super_ctx))
            }
        }
    }

    fn get_mut_super_ctx<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Ctx, bool) -> T,
    {
        let Some(name_or_ref) = &self.super_ctx else {
            return f(self, false);
        };
        match name_or_ref {
            Either::Left(name) => {
                let Some(TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), tag }) = self.name_map.get_mut(name) else {
                    return f(self, false);
                };
                if matches!(tag, InvariantTag::Const) {
                    return f(self, false);
                }
                f(super_ctx, true)
            }
            Either::Right(RefVal(r)) => {
                let Ok(mut o) = Keeper::owner(r) else {
                    return f(self, false);
                };
                let TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), tag } = &mut *o else {
                    return f(self, false);
                };
                if matches!(tag, InvariantTag::Const) {
                    return f(self, false);
                }
                f(super_ctx, true)
            }
        }
    }

    pub(crate) fn get_ref_or_val<F>(&self, name: Val, f: F) -> Val
    where
        F: FnOnce(Either<&Val, Val>) -> Val,
    {
        match name {
            Val::Symbol(s) => self.get_ref(&s, |op_val| {
                let Some(val) = op_val else {
                    return Val::default();
                };
                if let Val::Ref(RefVal(r)) = val {
                    let Ok(r) = Keeper::reader(r) else {
                        return Val::default();
                    };
                    f(Either::Left(&r.val))
                } else {
                    f(Either::Left(val))
                }
            }),
            Val::Ref(RefVal(r)) => {
                let Ok(r) = Keeper::reader(&r) else {
                    return Val::default();
                };
                f(Either::Left(&r.val))
            }
            val => f(Either::Right(val)),
        }
    }

    pub(crate) fn get_mut_or_val<F>(&mut self, name: Val, f: F) -> Val
    where
        F: FnOnce(Either<&mut Val, Val>) -> Val,
    {
        match name {
            Val::Symbol(s) => self.get_mut(&s, |op_val| {
                let Some(val) = op_val else {
                    return Val::default();
                };
                if let Val::Ref(RefVal(r)) = val {
                    let Ok(mut o) = Keeper::owner(r) else {
                        return Val::default();
                    };
                    if matches!(o.tag, InvariantTag::Const) {
                        return Val::default();
                    }
                    f(Either::Left(&mut o.val))
                } else {
                    f(Either::Left(val))
                }
            }),
            Val::Ref(RefVal(r)) => {
                let Ok(mut o) = Keeper::owner(&r) else {
                    return Val::default();
                };
                if matches!(o.tag, InvariantTag::Const) {
                    return Val::default();
                }
                f(Either::Left(&mut o.val))
            }
            val => f(Either::Right(val)),
        }
    }
}

impl Debug for PrimitiveEval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveEval::CtxFree {
                eval_mode,
                evaluator: eval,
            } => {
                let ptr: *const dyn Fn(Val) -> Val = &**eval;
                f.debug_struct("PrimitiveEval::CtxFree")
                    .field("eval_mode", eval_mode)
                    .field("eval", &format!("{ptr:p}"))
                    .finish()
            }
            PrimitiveEval::CtxAware { evaluator: eval } => {
                let ptr: *const dyn Fn(&mut Ctx, Val) -> Val = &**eval;
                f.debug_struct("PrimitiveEval::CtxAware")
                    .field("eval", &format!("{ptr:p}"))
                    .finish()
            }
        }
    }
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Primitive {}

impl Hash for Primitive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TaggedVal {
    pub(crate) fn new(val: Val) -> TaggedVal {
        TaggedVal {
            tag: InvariantTag::None,
            val,
        }
    }
    pub(crate) fn new_final(val: Val) -> TaggedVal {
        TaggedVal {
            tag: InvariantTag::Final,
            val,
        }
    }
    pub(crate) fn new_const(val: Val) -> TaggedVal {
        TaggedVal {
            tag: InvariantTag::Const,
            val,
        }
    }
}

impl Func {
    pub(crate) fn new_primitive(primitive: Primitive) -> Func {
        Func {
            func_trait: FuncTrait {},
            func_impl: FuncImpl::Primitive(primitive),
        }
    }

    pub(crate) fn new_composed(composed: Composed) -> Func {
        Func {
            func_trait: FuncTrait {},
            func_impl: FuncImpl::Composed(composed),
        }
    }
}

impl Primitive {
    pub(crate) fn new_ctx_free(
        id: &str,
        eval_mode: EvalMode,
        evaluator: impl Fn(Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            eval: PrimitiveEval::CtxFree {
                eval_mode,
                evaluator: Reader::new(evaluator),
            },
        }
    }

    pub(crate) fn new_ctx_aware(
        id: &str,
        evaluator: impl Fn(&mut Ctx, Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            eval: PrimitiveEval::CtxAware {
                evaluator: Reader::new(evaluator),
            },
        }
    }
}
