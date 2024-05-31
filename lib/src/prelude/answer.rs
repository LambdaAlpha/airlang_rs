use std::ops::Deref;

use crate::{
    ctx::{
        constant::CtxForConstFn,
        CtxMap,
        DefaultCtx,
    },
    prelude::{
        default_mode,
        named_const_fn,
        named_free_fn,
        symbol_id_mode,
        Named,
        Prelude,
    },
    problem::Verified,
    Answer,
    AnswerVal,
    Bool,
    FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct AnswerPrelude {
    pub(crate) unsolved: Named<AnswerVal>,
    pub(crate) unsolvable: Named<AnswerVal>,
    pub(crate) unverified: Named<FuncVal>,
    pub(crate) verified: Named<FuncVal>,
    pub(crate) is_unsolved: Named<FuncVal>,
    pub(crate) is_unsolvable: Named<FuncVal>,
    pub(crate) is_verified: Named<FuncVal>,
    pub(crate) input: Named<FuncVal>,
    pub(crate) into_input: Named<FuncVal>,
    pub(crate) evidence: Named<FuncVal>,
    pub(crate) into_evidence: Named<FuncVal>,
}

impl Default for AnswerPrelude {
    fn default() -> Self {
        AnswerPrelude {
            unsolved: unsolved(),
            unsolvable: unsolvable(),
            unverified: unverified(),
            verified: verified(),
            is_unsolved: is_unsolved(),
            is_unsolvable: is_unsolvable(),
            is_verified: is_verified(),
            input: input(),
            into_input: into_input(),
            evidence: evidence(),
            into_evidence: into_evidence(),
        }
    }
}

impl Prelude for AnswerPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.unsolved.put(m);
        self.unsolvable.put(m);
        self.unverified.put(m);
        self.verified.put(m);
        self.is_unsolved.put(m);
        self.is_unsolvable.put(m);
        self.is_verified.put(m);
        self.input.put(m);
        self.into_input.put(m);
        self.evidence.put(m);
        self.into_evidence.put(m);
    }
}

fn unsolved() -> Named<AnswerVal> {
    Named::new("answer.unsolved", AnswerVal::from(Answer::Unsolved))
}

fn unsolvable() -> Named<AnswerVal> {
    Named::new("answer.unsolvable", AnswerVal::from(Answer::Unsolvable))
}

fn unverified() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("answer.unverified", input_mode, output_mode, fn_unverified)
}

fn fn_unverified(input: Val) -> Val {
    Val::Answer(AnswerVal::from(Answer::Unverified(input)))
}

fn verified() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("answer.verified", input_mode, output_mode, fn_verified)
}

fn fn_verified(input: Val) -> Val {
    let Val::Assert(assert) = input else {
        return Val::default();
    };
    if !assert.is_verified() {
        return Val::default();
    }
    Val::Answer(AnswerVal::from(Answer::Verified(
        Verified::new(assert).unwrap(),
    )))
}

fn is_unsolved() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn(
        "answer.is_unsolved",
        input_mode,
        output_mode,
        fn_is_unsolved,
    )
}

fn fn_is_unsolved(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let is_unknown = matches!(&**answer, Answer::Unsolved);
        Val::Bool(Bool::new(is_unknown))
    })
}

fn is_unsolvable() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn(
        "answer.is_unsolvable",
        input_mode,
        output_mode,
        fn_is_unsolvable,
    )
}

fn fn_is_unsolvable(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let is_unsolvable = matches!(&**answer, Answer::Unsolvable);
        Val::Bool(Bool::new(is_unsolvable))
    })
}

fn is_verified() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn(
        "answer.is_verified",
        input_mode,
        output_mode,
        fn_is_verified,
    )
}

fn fn_is_verified(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        match &**answer {
            Answer::Unverified(_) => Val::Bool(Bool::f()),
            Answer::Verified(_) => Val::Bool(Bool::t()),
            _ => Val::default(),
        }
    })
}

fn input() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("answer.input", input_mode, output_mode, fn_input)
}

fn fn_input(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        match &**answer {
            Answer::Unsolved => Val::default(),
            Answer::Unsolvable => Val::default(),
            Answer::Unverified(value) => value.clone(),
            Answer::Verified(assert) => assert.input().clone(),
        }
    })
}

fn into_input() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("answer.into_input", input_mode, output_mode, fn_into_input)
}

fn fn_into_input(input: Val) -> Val {
    let Val::Answer(answer) = input else {
        return Val::default();
    };
    let answer = Answer::from(answer);
    match answer {
        Answer::Unsolved => Val::default(),
        Answer::Unsolvable => Val::default(),
        Answer::Unverified(value) => value,
        Answer::Verified(assert) => assert.input().clone(),
    }
}

fn evidence() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("answer.evidence", input_mode, output_mode, fn_evidence)
}

fn fn_evidence(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let Answer::Verified(assert) = &**answer else {
            return Val::default();
        };
        Val::Assert(assert.deref().clone())
    })
}

fn into_evidence() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn(
        "answer.into_evidence",
        input_mode,
        output_mode,
        fn_into_evidence,
    )
}

fn fn_into_evidence(input: Val) -> Val {
    let Val::Answer(answer) = input else {
        return Val::default();
    };
    let answer = Answer::from(answer);
    let Answer::Verified(verified) = answer else {
        return Val::default();
    };
    Val::Assert(verified.unwrap())
}
