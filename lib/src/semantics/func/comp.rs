use std::mem::take;

use super::MutFn;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::memo::Contract;
use crate::semantics::memo::Memo;
use crate::semantics::memo::MemoError;
use crate::semantics::memo::MemoValue;
use crate::semantics::val::Val;
use crate::type_::DynRef;
use crate::type_::Key;
use crate::utils::gurad::guard;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct FreeComposite {
    pub(crate) body: Val,
    pub(crate) input_name: Key,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct DynComposite {
    pub(crate) free: FreeComposite,
    pub(crate) ctx_name: Key,
}

impl FreeComposite {
    pub(super) fn call(&self, cfg: &mut Cfg, memo: &mut Memo, input: Val) -> Val {
        if put_input(memo, self.input_name.clone(), input).is_err() {
            return Val::default();
        }
        composite_call(cfg, memo, self.body.clone())
    }
}

impl DynComposite {
    pub(super) fn call(&self, cfg: &mut Cfg, memo: &mut Memo, ctx: DynRef<Val>, input: Val) -> Val {
        if put_input(memo, self.free.input_name.clone(), input).is_err() {
            return Val::default();
        }
        let eval = |inner: &mut Memo| composite_call(cfg, inner, self.free.body.clone());
        with_ctx(memo, ctx, self.ctx_name.clone(), eval)
    }
}

pub(crate) fn composite_call(cfg: &mut Cfg, memo: &mut Memo, body: Val) -> Val {
    let memo_val = Val::Memo(take(memo).into());
    guard(
        (cfg, memo_val),
        move |(cfg, memo_val)| Eval.mut_call(&mut **cfg, memo_val, body),
        |(_cfg, memo_val)| {
            let Val::Memo(memo_val) = memo_val else {
                unreachable!("composite_call ctx invariant is broken!!!")
            };
            *memo = memo_val.into();
        },
    )
}

fn put_input(memo: &mut Memo, input_name: Key, input: Val) -> Result<(), MemoError> {
    let _ = memo.put(input_name, input, Contract::None)?;
    Ok(())
}

fn with_ctx(
    memo: &mut Memo, mut ctx: DynRef<Val>, name: Key, f: impl FnOnce(&mut Memo) -> Val,
) -> Val {
    if memo.exist(name.clone()) {
        return Val::default();
    }
    keep_ctx(memo, ctx.reborrow(), name.clone());
    guard(
        (memo, ctx, name),
        move |(memo, _ctx, _name)| f(memo),
        |(memo, ctx, name)| {
            restore_ctx(memo, ctx.unwrap(), name);
        },
    )
}

fn keep_ctx(memo: &mut Memo, ctx: DynRef<Val>, name: Key) {
    let const_ = ctx.is_const();
    // here is why we need a `&mut Val` for a const func
    let ctx = take(ctx.unwrap());
    let contract = if const_ { Contract::Const } else { Contract::Static };
    let _ = memo.put_unchecked(name, MemoValue::new(ctx, contract));
}

fn restore_ctx(memo: &mut Memo, ctx: &mut Val, name: Key) {
    let Some(ctx_val) = memo.remove_unchecked(&name) else {
        unreachable!("restore_ctx ctx invariant is broken!!!");
    };
    *ctx = ctx_val.val;
}
