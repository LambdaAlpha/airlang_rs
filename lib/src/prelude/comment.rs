use std::mem::swap;

use crate::{
    ctx::{
        ref1::CtxMeta,
        CtxMap,
        DefaultCtx,
    },
    func::MutableDispatcher,
    prelude::{
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        Named,
        Prelude,
    },
    syntax::COMMENT_INFIX,
    transform::eval::Eval,
    transformer::input::ByVal,
    types::either::Either,
    Comment,
    CtxForConstFn,
    CtxForMutableFn,
    FreeCtx,
    FuncVal,
    Mode,
    Pair,
    Val,
};

#[derive(Clone)]
pub(crate) struct CommentPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_note: Named<FuncVal>,
    pub(crate) set_note: Named<FuncVal>,
    pub(crate) get_value: Named<FuncVal>,
    pub(crate) set_value: Named<FuncVal>,
}

impl Default for CommentPrelude {
    fn default() -> Self {
        CommentPrelude {
            new: new(),
            apply: apply(),
            get_note: get_note(),
            set_note: set_note(),
            get_value: get_value(),
            set_value: set_value(),
        }
    }
}

impl Prelude for CommentPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.apply.put(m);
        self.get_note.put(m);
        self.set_note.put(m);
        self.get_value.put(m);
        self.set_value.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn(COMMENT_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Comment(Comment::new(pair.first, pair.second).into())
}

fn apply() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    let func = MutableDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mutable_fn("comment.apply", input_mode, output_mode, func)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Comment(comment) = input else {
        return Val::default();
    };
    Eval.transform_comment(ctx, comment)
}

fn get_note() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("comment.note", input_mode, output_mode, fn_get_note)
}

fn fn_get_note(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Comment(comment) => comment.note.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Comment(comment) => Comment::from(comment).note,
            _ => Val::default(),
        },
    })
}

fn set_note() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mutable_fn("comment.set_note", input_mode, output_mode, fn_set_note)
}

fn fn_set_note(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut comment) => {
            let Some(Val::Comment(comment)) = comment.as_mut() else {
                return Val::default();
            };
            swap(&mut comment.note, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

fn get_value() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("comment.value", input_mode, output_mode, fn_get_value)
}

fn fn_get_value(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Comment(comment) => comment.value.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Comment(comment) => Comment::from(comment).value,
            _ => Val::default(),
        },
    })
}

fn set_value() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mutable_fn("comment.set_value", input_mode, output_mode, fn_set_value)
}

fn fn_set_value(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut comment) => {
            let Some(Val::Comment(comment)) = comment.as_mut() else {
                return Val::default();
            };
            swap(&mut comment.value, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
