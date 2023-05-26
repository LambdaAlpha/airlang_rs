use {
    crate::{
        semantics::val::{
            ListVal,
            MapVal,
            Val,
        },
        types::{
            Either,
            Keeper,
            Map,
            Pair,
            Reader,
        },
    },
    smartstring::alias::CompactString,
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::Hash,
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

#[derive(Clone, Hash)]
pub(crate) struct Primitive {
    pub(crate) id: Name,
    pub(crate) eval: Reader<dyn Fn(&mut Ctx, Val) -> Val>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed {
    pub(crate) body: Val,
    pub(crate) constants: NameMap,
    pub(crate) input_name: Option<Name>,
    pub(crate) caller_name: Option<Name>,
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
    pub(crate) fn eval(self, ctx: &mut Ctx, input: Val) -> Val {
        let mut name_map = self.constants;
        if let Some(input_name) = self.input_name {
            name_map.insert(input_name, TaggedVal::new(input));
        }
        if let Some(caller_name) = &self.caller_name {
            let mut ctx_swap = Ctx::default();
            swap(ctx, &mut ctx_swap);
            name_map.insert(
                caller_name.clone(),
                TaggedVal::new_final(Val::Ctx(Box::new(ctx_swap))),
            );
        }
        let reverse_interpreter = ctx.reverse_interpreter.clone();

        let mut new_ctx = Ctx {
            name_map,
            reverse_interpreter,
        };
        let output = new_ctx.eval(self.body);
        if let Some(caller_name) = &self.caller_name {
            if let Val::Ctx(caller) = new_ctx.into_val(caller_name) {
                *ctx = *caller;
            }
        }
        output
    }
}

impl Ctx {
    pub(crate) fn eval(&mut self, input: Val) -> Val {
        match input {
            Val::Symbol(s) => self.get(&s),
            Val::Box(k) => self.eval_box(&k.0),
            Val::Pair(p) => self.eval_pair(p.first, p.second),
            Val::List(l) => self.eval_list(l),
            Val::Map(m) => self.eval_map(m),
            Val::Call(c) => self.eval_call(c.func, c.input),
            Val::Reverse(r) => self.eval_reverse(r.func, r.output),
            v => v,
        }
    }

    pub(crate) fn eval_box(&self, keeper: &Keeper<TaggedVal>) -> Val {
        if let Ok(input) = Keeper::reader(&keeper) {
            input.deref().val.clone()
        } else {
            Val::default()
        }
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
        if let Val::Func(func) = self.eval(func) {
            func.eval(self, input)
        } else {
            Val::default()
        }
    }

    pub(crate) fn eval_reverse(&mut self, func: Val, output: Val) -> Val {
        let reverse_interpreter = self.reverse_interpreter.clone();
        if let Some(reverse_interpreter) = reverse_interpreter {
            let reverse_func = reverse_interpreter.deref().clone().eval(self, func);
            self.eval_call(reverse_func, output)
        } else {
            Val::default()
        }
    }

    pub(crate) fn get(&self, name: &str) -> Val {
        self.name_map
            .get(name)
            .map(|tagged_val| tagged_val.val.clone())
            .unwrap_or_default()
    }

    pub(crate) fn get_ref(&self, name: &str) -> Option<&Val> {
        self.name_map.get(name).map(|tagged_val| &tagged_val.val)
    }

    pub(crate) fn get_mut(&mut self, name: &str) -> Option<&mut Val> {
        self.name_map
            .get_mut(name)
            .and_then(|tagged_val| match tagged_val.tag {
                InvariantTag::Const => None,
                _ => Some(&mut tagged_val.val),
            })
    }

    pub(crate) fn remove(&mut self, name: &str) -> Val {
        if let Some(tagged_val) = self.name_map.get(name) {
            if matches!(&tagged_val.tag, InvariantTag::None) {
                return self
                    .name_map
                    .remove(name)
                    .map(|tagged_val| tagged_val.val)
                    .unwrap_or_default();
            }
        }
        Val::default()
    }

    fn into_val(mut self, name: &str) -> Val {
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn put_val(&mut self, name: Name, val: TaggedVal) -> Val {
        if let Some(tagged_val) = self.name_map.get(&name) {
            if matches!(&tagged_val.tag, InvariantTag::None) {
                self.name_map
                    .insert(name, val)
                    .map(|tagged_val| tagged_val.val)
                    .unwrap_or_default()
            } else {
                Val::default()
            }
        } else {
            self.name_map
                .insert(name, val)
                .map(|ctx_val| ctx_val.val)
                .unwrap_or_default()
        }
    }

    pub(crate) fn set_final(&mut self, name: &str) {
        if let Some(tagged_val) = self.name_map.get_mut(name) {
            if matches!(&tagged_val.tag, InvariantTag::None) {
                tagged_val.tag = InvariantTag::Final;
            }
        }
    }

    pub(crate) fn set_const(&mut self, name: &str) {
        if let Some(tagged_val) = self.name_map.get_mut(name) {
            tagged_val.tag = InvariantTag::Const;
        }
    }

    pub(crate) fn eval_ref<M, F, G>(&self, name: Val, map: M) -> Val
    where
        F: FnOnce(&Val) -> Val,
        G: FnOnce(Val) -> Val,
        M: FnOnce(bool) -> Either<F, G>,
    {
        match name {
            Val::Symbol(s) => {
                if let Some(val) = self.get_ref(&s) {
                    if let Either::Left(f) = map(true) {
                        return f(val);
                    }
                }
            }
            Val::String(s) => {
                if let Some(val) = self.get_ref(&s) {
                    if let Either::Left(f) = map(true) {
                        return f(val);
                    }
                }
            }
            Val::Box(k) => {
                if let Ok(r) = Keeper::reader(&k.0) {
                    if let Either::Left(f) = map(true) {
                        return f(&r.val);
                    }
                }
            }
            val => {
                if let Either::Right(g) = map(false) {
                    return g(val);
                }
            }
        }
        Val::default()
    }

    pub(crate) fn eval_mut<M, F, G>(&mut self, name: Val, map: M) -> Val
    where
        F: FnOnce(&mut Val) -> Val,
        G: FnOnce(Val) -> Val,
        M: FnOnce(bool) -> Either<F, G>,
    {
        match name {
            Val::Symbol(s) => {
                if let Some(val) = self.get_mut(&s) {
                    if let Either::Left(f) = map(true) {
                        return f(val);
                    }
                }
            }
            Val::String(s) => {
                if let Some(val) = self.get_mut(&s) {
                    if let Either::Left(f) = map(true) {
                        return f(val);
                    }
                }
            }
            Val::Box(k) => {
                if let Ok(mut o) = Keeper::owner(&k.0) {
                    if matches!(o.tag, InvariantTag::None | InvariantTag::Final) {
                        if let Either::Left(f) = map(true) {
                            return f(&mut o.val);
                        }
                    }
                }
            }
            val => {
                if let Either::Right(g) = map(false) {
                    return g(val);
                }
            }
        }
        Val::default()
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
