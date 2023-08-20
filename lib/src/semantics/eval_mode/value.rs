use crate::{
    semantics::{
        eval::{
            input::{
                ByRef,
                ByVal,
            },
            DefaultByRef,
            DefaultByVal,
            Evaluator,
        },
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
    },
    types::{
        Call,
        Pair,
        Reverse,
        Symbol,
    },
};

pub(crate) struct Value;

impl<Ctx> Evaluator<Ctx, Val, Val> for Value {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Value {
    fn eval_atoms(&self, _ctx: &mut Ctx, input: Val) -> Val {
        input
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, s: Symbol) -> Val {
        Val::Symbol(s)
    }

    fn eval_ref(&self, _ctx: &mut Ctx, ref_val: RefVal) -> Val {
        Val::Ref(ref_val)
    }

    fn eval_pair(&self, _ctx: &mut Ctx, first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn eval_list(&self, _ctx: &mut Ctx, list: ListVal) -> Val {
        Val::List(list)
    }

    fn eval_map(&self, _ctx: &mut Ctx, map: MapVal) -> Val {
        Val::Map(map)
    }

    fn eval_call(&self, _ctx: &mut Ctx, func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

pub(crate) struct ValueByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for ValueByRef {
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for ValueByRef {
    fn eval_atoms(&self, _ctx: &mut Ctx, input: &'a Val) -> Val {
        input.clone()
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, s: &'a Symbol) -> Val {
        Val::Symbol(s.clone())
    }

    fn eval_ref(&self, _ctx: &mut Ctx, ref_val: &'a RefVal) -> Val {
        Val::Ref(ref_val.clone())
    }

    fn eval_pair(&self, _ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        Val::Pair(Box::new(Pair::new(first.clone(), second.clone())))
    }

    fn eval_list(&self, _ctx: &mut Ctx, list: &'a ListVal) -> Val {
        Val::List(list.clone())
    }

    fn eval_map(&self, _ctx: &mut Ctx, map: &'a MapVal) -> Val {
        Val::Map(map.clone())
    }

    fn eval_call(&self, _ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        Val::Call(Box::new(Call::new(func.clone(), input.clone())))
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func.clone(), output.clone())))
    }
}

pub(crate) struct ValueFreeConst;

impl<Ctx> Evaluator<Ctx, Val, Option<Val>> for ValueFreeConst {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Option<Val> {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Option<Val>> for ValueFreeConst {
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Option<Val> {
        Some(Value.eval_atoms(ctx, input))
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Option<Val> {
        Some(Value.eval_symbol(ctx, s))
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Option<Val> {
        Some(Value.eval_ref(ctx, ref_val))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Option<Val> {
        Some(Value.eval_pair(ctx, first, second))
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Option<Val> {
        Some(Value.eval_list(ctx, list))
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Option<Val> {
        Some(Value.eval_map(ctx, map))
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Option<Val> {
        Some(Value.eval_call(ctx, func, input))
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Option<Val> {
        Some(Value.eval_reverse(ctx, func, output))
    }
}

pub(crate) struct ValueFreeConstByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Option<Val>> for ValueFreeConstByRef {
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Option<Val> {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Option<Val>> for ValueFreeConstByRef {
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Option<Val> {
        Some(ValueByRef.eval_atoms(ctx, input))
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Option<Val> {
        Some(ValueByRef.eval_symbol(ctx, s))
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &'a RefVal) -> Option<Val> {
        Some(ValueByRef.eval_ref(ctx, ref_val))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Option<Val> {
        Some(ValueByRef.eval_pair(ctx, first, second))
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Option<Val> {
        Some(ValueByRef.eval_list(ctx, list))
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Option<Val> {
        Some(ValueByRef.eval_map(ctx, map))
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Option<Val> {
        Some(ValueByRef.eval_call(ctx, func, input))
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Option<Val> {
        Some(ValueByRef.eval_reverse(ctx, func, output))
    }
}

pub(crate) struct ValueFreeConstChecker;

impl<Ctx> Evaluator<Ctx, Val, bool> for ValueFreeConstChecker {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> bool {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, bool> for ValueFreeConstChecker {
    fn eval_atoms(&self, _ctx: &mut Ctx, _input: Val) -> bool {
        true
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: Symbol) -> bool {
        true
    }

    fn eval_ref(&self, _ctx: &mut Ctx, _ref_val: RefVal) -> bool {
        true
    }

    fn eval_pair(&self, _ctx: &mut Ctx, _first: Val, _second: Val) -> bool {
        true
    }

    fn eval_list(&self, _ctx: &mut Ctx, _list: ListVal) -> bool {
        true
    }

    fn eval_map(&self, _ctx: &mut Ctx, _map: MapVal) -> bool {
        true
    }

    fn eval_call(&self, _ctx: &mut Ctx, _func: Val, _input: Val) -> bool {
        true
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: Val, _output: Val) -> bool {
        true
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, bool> for ValueFreeConstChecker {
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> bool {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, bool> for ValueFreeConstChecker {
    fn eval_atoms(&self, _ctx: &mut Ctx, _input: &'a Val) -> bool {
        true
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: &'a Symbol) -> bool {
        true
    }

    fn eval_ref(&self, _ctx: &mut Ctx, _ref_val: &'a RefVal) -> bool {
        true
    }

    fn eval_pair(&self, _ctx: &mut Ctx, _first: &'a Val, _second: &'a Val) -> bool {
        true
    }

    fn eval_list(&self, _ctx: &mut Ctx, _list: &'a ListVal) -> bool {
        true
    }

    fn eval_map(&self, _ctx: &mut Ctx, _map: &'a MapVal) -> bool {
        true
    }

    fn eval_call(&self, _ctx: &mut Ctx, _func: &'a Val, _input: &'a Val) -> bool {
        true
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: &'a Val, _output: &'a Val) -> bool {
        true
    }
}
