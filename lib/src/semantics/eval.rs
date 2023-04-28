use {
    crate::{
        semantics::val::{
            CallVal,
            ReverseVal,
            Val,
        },
        traits::TryClone,
        types::{
            ImRef,
            Map,
            Symbol,
        },
    },
    smartstring::alias::CompactString,
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        marker::PhantomData,
        mem::swap,
    },
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Func {
    pub(crate) func_trait: FuncTrait,
    pub(crate) func_impl: FuncImpl,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct FuncTrait {}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FuncImpl {
    Primitive(Primitive),
    Composed(Composed),
}

pub(crate) struct Primitive {
    pub(crate) id: Name,
    pub(crate) eval: ImRef<dyn Fn(&mut Ctx, Val) -> Val>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Composed {
    // is boxed to avoid infinite size of Val
    pub(crate) body: ImRef<Val>,
    pub(crate) constants: ImRef<NameMap>,
    pub(crate) input_name: Option<Name>,
    pub(crate) caller_name: Option<Name>,
}

pub(crate) type Name = CompactString;

pub(crate) type NameMap = Map<Name, Val>;

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Ctx {
    pub(crate) constants: ImRef<NameMap>,
    pub(crate) variables: NameMap,
    pub(crate) call_interpreter: PhantomData<fn(&Composed, &Val) -> Val>,
    pub(crate) reverse_interpreter: Option<ImRef<Func>>,
}

impl Func {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: &Val) -> Val {
        let val = ctx.eval_default(input);
        self.func_impl.eval(ctx, val)
    }
}

impl FuncImpl {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => (p.eval)(ctx, input),
            FuncImpl::Composed(c) => c.eval(ctx, input),
        }
    }
}

impl Composed {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        let constants = if let Some(constants) = self.constants.try_clone() {
            constants
        } else {
            return Val::default();
        };
        let mut variables = NameMap::default();
        if let Some(input_name) = &self.input_name {
            variables.insert(input_name.clone(), input);
        }
        if let Some(caller_name) = &self.caller_name {
            let mut ctx_swap = Ctx::default();
            swap(ctx, &mut ctx_swap);
            variables.insert(caller_name.clone(), Val::Ctx(ctx_swap));
        }
        let call_interpreter = PhantomData;
        let reverse_interpreter = ctx.reverse_interpreter.as_ref().and_then(|i| i.try_clone());

        let mut new_ctx = Ctx {
            constants,
            variables,
            call_interpreter,
            reverse_interpreter,
        };
        let output = new_ctx.eval(&self.body);
        if let Some(caller_name) = &self.caller_name {
            if let Val::Ctx(caller) = new_ctx.remove(caller_name) {
                *ctx = caller;
            }
        }
        output
    }
}

impl Ctx {
    pub(crate) fn eval(&mut self, input: &Val) -> Val {
        match input {
            Val::Symbol(s) => self.eval_symbol(s),
            Val::Call(c) => self.eval_call(c),
            Val::Reverse(r) => self.eval_reverse(r),
            v => self.eval_default(v),
        }
    }

    fn eval_default(&mut self, input: &Val) -> Val {
        input.try_clone().unwrap_or_default()
    }

    fn eval_symbol(&mut self, s: &Symbol) -> Val {
        self.get(s)
    }

    fn eval_call(&mut self, c: &CallVal) -> Val {
        self.eval_func_then_call(&c.func, &c.input)
    }

    fn eval_func_then_call(&mut self, func: &Val, input: &Val) -> Val {
        let func = if let Val::Func(f) = self.eval(func) {
            f
        } else {
            return Val::default();
        };
        func.eval(self, input)
    }

    fn eval_reverse(&mut self, r: &ReverseVal) -> Val {
        let reverse_interpreter = self
            .reverse_interpreter
            .as_ref()
            .and_then(|i| i.try_clone());
        if let Some(reverse_interpreter) = reverse_interpreter {
            let reverse_func = reverse_interpreter.eval(self, &r.func);
            self.eval_func_then_call(&reverse_func, &r.output)
        } else {
            Val::default()
        }
    }

    pub(crate) fn get(&self, name: &str) -> Val {
        self.constants
            .get(name)
            .or_else(|| self.variables.get(name))
            .and_then(|v| v.try_clone())
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

impl TryClone for Func {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Func {
            func_trait: self.func_trait.clone(),
            func_impl: self.func_impl.try_clone()?,
        })
    }
}

impl TryClone for FuncImpl {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(match self {
            FuncImpl::Primitive(p) => FuncImpl::Primitive(p.try_clone()?),
            FuncImpl::Composed(c) => FuncImpl::Composed(c.try_clone()?),
        })
    }
}

impl TryClone for Primitive {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Primitive {
            id: self.id.clone(),
            eval: self.eval.try_clone()?,
        })
    }
}

impl TryClone for Composed {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Composed {
            body: self.body.try_clone()?,
            constants: self.constants.try_clone()?,
            input_name: self.input_name.clone(),
            caller_name: self.caller_name.clone(),
        })
    }
}

impl TryClone for Ctx {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Ctx {
            constants: self.constants.try_clone()?,
            variables: self.variables.try_clone()?,
            call_interpreter: self.call_interpreter.clone(),
            reverse_interpreter: self
                .reverse_interpreter
                .as_ref()
                .and_then(|i| i.try_clone()),
        })
    }
}

impl TryClone for Name {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone())
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
