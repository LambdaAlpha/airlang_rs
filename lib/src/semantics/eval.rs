use {
    crate::{
        semantics::val::{
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

#[derive(Clone)]
pub(crate) struct Primitive {
    pub(crate) id: Name,
    #[allow(clippy::type_complexity)]
    pub(crate) eval: Reader<dyn Fn(&mut Ctx, Val) -> Val>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Name,
    pub(crate) caller_name: Name,
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
    pub(crate) super_ctx_name: Option<Name>,
    pub(crate) reverse_interpreter: Option<Reader<Func>>,
}

impl Func {
    pub(crate) fn eval(self, ctx: &mut Ctx, input: Val) -> Val {
        self.func_impl.eval(ctx, input)
    }
}

impl FuncImpl {
    pub(crate) fn eval(self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => (p.eval)(ctx, input),
            FuncImpl::Composed(c) => c.eval(ctx, input),
        }
    }
}

impl Composed {
    pub(crate) fn eval(mut self, ctx: &mut Ctx, input: Val) -> Val {
        self.ctx
            .put_val_local(self.input_name, TaggedVal::new(input));

        self.ctx.reverse_interpreter = ctx.reverse_interpreter.clone();

        let mut caller_ctx = Ctx::default();
        swap(ctx, &mut caller_ctx);
        self.ctx.put_val_local(
            self.caller_name.clone(),
            TaggedVal::new_final(Val::Ctx(Box::new(caller_ctx))),
        );

        let output = self.ctx.eval(self.body);

        if let Val::Ctx(caller) = self.ctx.into_val(&self.caller_name) {
            *ctx = *caller;
        }

        output
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
            .map(|(k, v)| (self.eval(k), self.eval(v)))
            .collect();
        Val::Map(map)
    }

    pub(crate) fn eval_call(&mut self, func: Val, input: Val) -> Val {
        let Val::Func(func) = self.eval(func) else {
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
            .map(|(k, v)| (self.eval_by_ref(k), self.eval_by_ref(v)))
            .collect();
        Val::Map(map)
    }

    pub(crate) fn eval_call_by_ref(&mut self, func: &Val, input: &Val) -> Val {
        let Val::Func(func) = self.eval_by_ref( func) else {
            return Val::default();
        };
        func.eval(self, input.clone())
    }

    pub(crate) fn eval_reverse_by_ref(&mut self, func: &Val, output: &Val) -> Val {
        self.eval_reverse(func.clone(), output.clone())
    }

    pub(crate) fn get(&self, name: &str) -> Val {
        self.name_map
            .get(name)
            .map(|tagged_val| tagged_val.val.clone())
            .or_else(|| self.get_ref_super_ctx().map(|ctx| ctx.get(name)))
            .unwrap_or_default()
    }

    pub(crate) fn get_ref(&self, name: &str) -> Option<&Val> {
        self.name_map
            .get(name)
            .map(|tagged_val| &tagged_val.val)
            .or_else(|| self.get_ref_super_ctx().and_then(|ctx| ctx.get_ref(name)))
    }

    pub(crate) fn get_mut(&mut self, name: &str) -> Option<&mut Val> {
        if self.name_map.get(name).is_none() {
            return self.get_mut_super_ctx().and_then(|ctx| ctx.get_mut(name));
        }
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            return None;
        };
        match tagged_val.tag {
            InvariantTag::Const => None,
            _ => Some(&mut tagged_val.val),
        }
    }

    pub(crate) fn remove(&mut self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_mut_super_ctx().map(|ctx| ctx.remove(name)).unwrap_or_default();
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    fn into_val(mut self, name: &str) -> Val {
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn put_val(&mut self, name: Name, val: TaggedVal) -> Val {
        match self.name_map.get(&name) {
            None => {
                if let Some(super_ctx) = self.get_mut_super_ctx() {
                    super_ctx.put_val(name, val)
                } else {
                    self.put_unchecked(name, val)
                }
            }
            Some(tagged_val) => {
                if !matches!(&tagged_val.tag, InvariantTag::None) {
                    return Val::default();
                }
                self.put_unchecked(name, val)
            }
        }
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
            if let Some(ctx) = self.get_mut_super_ctx() {
                ctx.set_final(name);
            }
            return;
        };
        if !(matches!(&tagged_val.tag, InvariantTag::None)) {
            return;
        }
        tagged_val.tag = InvariantTag::Final;
    }

    pub(crate) fn set_const(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            if let Some(ctx) = self.get_mut_super_ctx() {
                ctx.set_const(name);
            }
            return;
        };
        tagged_val.tag = InvariantTag::Const;
    }

    pub(crate) fn is_final(&self, name: &str) -> bool {
        self.name_map
            .get(name)
            .map(|tagged_val| matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const))
            .or_else(|| self.get_ref_super_ctx().map(|ctx| ctx.is_final(name)))
            .unwrap_or_default()
    }

    pub(crate) fn is_const(&self, name: &str) -> bool {
        self.name_map
            .get(name)
            .map(|tagged_val| matches!(&tagged_val.tag, InvariantTag::Const))
            .or_else(|| self.get_ref_super_ctx().map(|ctx| ctx.is_const(name)))
            .unwrap_or_default()
    }

    fn get_ref_super_ctx(&self) -> Option<&Ctx> {
        let Some(name) = &self.super_ctx_name else {
            return None;
        };
        let Some(TaggedVal { val: Val::Ctx(super_ctx), .. }) = self.name_map.get(name) else {
            return None;
        };
        Some(super_ctx)
    }

    fn get_mut_super_ctx(&mut self) -> Option<&mut Ctx> {
        let Some(name) = &self.super_ctx_name else {
            return None;
        };
        let Some(TaggedVal { val: Val::Ctx(super_ctx), tag }) = self.name_map.get_mut(name) else {
            return None;
        };
        if matches!(tag, InvariantTag::Const) {
            return None;
        }
        Some(super_ctx)
    }

    pub(crate) fn get_ref_or_val<F>(&self, name: Val, f: F) -> Val
    where
        F: FnOnce(Either<&Val, Val>) -> Val,
    {
        match name {
            Val::Symbol(s) => {
                let Some(val) = self.get_ref(&s) else {
                    return Val::default();
                };
                f(Either::Left(val))
            }
            Val::Ref(k) => {
                let Ok(r) = Keeper::reader(&k.0) else {
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
            Val::Symbol(s) => {
                let Some(val) = self.get_mut(&s) else {
                    return Val::default();
                };
                f(Either::Left(val))
            }
            Val::Ref(k) => {
                let Ok(mut o) = Keeper::owner(&k.0) else {
                    return Val::default();
                };
                if !matches!(o.tag, InvariantTag::None | InvariantTag::Final) {
                    return Val::default();
                }
                f(Either::Left(&mut o.val))
            }
            val => f(Either::Right(val)),
        }
    }

    pub(crate) fn eval_escape(&mut self, input: Val) -> Val {
        match input {
            Val::Call(c) => match &c.func {
                Val::Symbol(s) => {
                    if &**s == "\\" {
                        self.eval(c.input)
                    } else {
                        let func = self.eval_escape(c.func);
                        let input = self.eval_escape(c.input);
                        let call = Box::new(Call::new(func, input));
                        Val::Call(call)
                    }
                }
                _ => {
                    let func = self.eval_escape(c.func);
                    let input = self.eval_escape(c.input);
                    let call = Box::new(Call::new(func, input));
                    Val::Call(call)
                }
            },
            Val::Pair(p) => {
                let first = self.eval_escape(p.first);
                let second = self.eval_escape(p.second);
                let pair = Box::new(Pair::new(first, second));
                Val::Pair(pair)
            }
            Val::Reverse(r) => {
                let func = self.eval_escape(r.func);
                let output = self.eval_escape(r.output);
                let reverse = Box::new(Reverse::new(func, output));
                Val::Reverse(reverse)
            }
            Val::List(l) => {
                let list = l.into_iter().map(|v| self.eval_escape(v)).collect();
                Val::List(list)
            }
            Val::Map(m) => {
                let map = m
                    .into_iter()
                    .map(|(k, v)| {
                        let key = self.eval_escape(k);
                        let value = self.eval_escape(v);
                        (key, value)
                    })
                    .collect();
                Val::Map(map)
            }
            i => i,
        }
    }
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ptr: *const dyn Fn(&mut Ctx, Val) -> Val = &*self.eval;
        f.debug_struct("Primitive")
            .field("id", &self.id)
            .field("eval", &format!("{ptr:p}"))
            .finish()
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
