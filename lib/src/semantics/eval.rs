use {
    crate::{
        semantics::val::{
            KeeperVal,
            ListVal,
            MapVal,
            Val,
        },
        types::{
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
    pub(crate) constants: Reader<NameMap>,
    pub(crate) input_name: Option<Name>,
    pub(crate) caller_name: Option<Name>,
}

pub(crate) type Name = CompactString;

pub(crate) type NameMap = Map<Name, Val>;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Ctx {
    pub(crate) constants: Reader<NameMap>,
    pub(crate) variables: NameMap,
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
        let constants = self.constants;
        let mut variables = NameMap::default();
        if let Some(input_name) = self.input_name {
            variables.insert(input_name, input);
        }
        if let Some(caller_name) = &self.caller_name {
            let mut ctx_swap = Ctx::default();
            swap(ctx, &mut ctx_swap);
            variables.insert(caller_name.clone(), Val::Ctx(Box::new(ctx_swap)));
        }
        let reverse_interpreter = ctx.reverse_interpreter.clone();

        let mut new_ctx = Ctx {
            constants,
            variables,
            reverse_interpreter,
        };
        let output = new_ctx.eval(self.body);
        if let Some(caller_name) = &self.caller_name {
            if let Val::Ctx(caller) = new_ctx.remove(caller_name) {
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
            Val::Keeper(k) => self.eval_keeper(&k),
            Val::Pair(p) => self.eval_pair(p.first, p.second),
            Val::List(l) => self.eval_list(l),
            Val::Map(m) => self.eval_map(m),
            Val::Call(c) => self.eval_call(c.func, c.input),
            Val::Reverse(r) => self.eval_reverse(r.func, r.output),
            v => v,
        }
    }

    pub(crate) fn eval_keeper(&self, keeper: &KeeperVal) -> Val {
        if let Ok(input) = Keeper::reader(&keeper) {
            input.deref().clone()
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
        self.constants
            .get(name)
            .or_else(|| self.variables.get(name))
            .map(Clone::clone)
            .unwrap_or_default()
    }

    pub(crate) fn remove(&mut self, name: &str) -> Val {
        self.constants
            .get(name)
            .map(|_| Val::default())
            .or_else(|| self.variables.remove(name))
            .unwrap_or_default()
    }

    pub(crate) fn put(&mut self, name: Name, val: Val) -> Val {
        self.constants
            .get(&name)
            .map(|_| Val::default())
            .or_else(|| self.variables.insert(name, val))
            .unwrap_or_default()
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
