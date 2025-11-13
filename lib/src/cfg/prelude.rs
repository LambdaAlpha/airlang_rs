use crate::cfg::lib::CoreLib;
use crate::cfg::lib::adapter::CODE_EVAL;
use crate::cfg::lib::adapter::CODE_ID;
use crate::cfg::lib::adapter::CODE_LITERAL;
use crate::cfg::lib::adapter::CODE_REF;
use crate::cfg::lib::adapter::CallPrimAdapter;
use crate::cfg::lib::adapter::DATA_EVAL;
use crate::cfg::lib::adapter::DATA_ID;
use crate::cfg::lib::adapter::DATA_LITERAL;
use crate::cfg::lib::adapter::DATA_REF;
use crate::cfg::lib::adapter::SymbolAdapter;
use crate::cfg::lib::adapter::adapter_free_func;
use crate::cfg::lib::adapter::adapter_mut_func;
use crate::cfg::lib::adapter::prim_adapter;
use crate::semantics::memo::Contract;
use crate::semantics::memo::Memo;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct CorePrelude {
    pub not: FreePrimFuncVal,
    pub and: FreePrimFuncVal,
    pub or: FreePrimFuncVal,
    pub xor: FreePrimFuncVal,
    pub imply: FreePrimFuncVal,

    pub add: FreePrimFuncVal,
    pub subtract: FreePrimFuncVal,
    pub multiply: FreePrimFuncVal,
    pub divide: FreePrimFuncVal,
    pub less_than: FreePrimFuncVal,
    pub less_equal: FreePrimFuncVal,
    pub greater_than: FreePrimFuncVal,
    pub greater_equal: FreePrimFuncVal,

    pub call: FreePrimFuncVal,

    pub exist: FreePrimFuncVal,
    pub import: FreePrimFuncVal,
    pub export: FreePrimFuncVal,
    pub with: MutPrimFuncVal,

    pub move_: MutPrimFuncVal,

    pub function: FreePrimFuncVal,

    pub get: ConstPrimFuncVal,
    pub set: MutPrimFuncVal,
    pub assign: MutPrimFuncVal,
    pub which: MutPrimFuncVal,

    pub do_: MutPrimFuncVal,
    pub test: MutPrimFuncVal,
    pub switch: MutPrimFuncVal,
    pub match_: MutPrimFuncVal,
    pub loop_: MutPrimFuncVal,
    pub iterate: MutPrimFuncVal,

    pub type_: ConstPrimFuncVal,
    pub equal: FreePrimFuncVal,

    pub eval: MutPrimFuncVal,

    pub form_id: FreePrimFuncVal,
    pub form_literal: MutPrimFuncVal,
    pub form_ref: MutPrimFuncVal,
    pub form_eval: MutPrimFuncVal,
    pub eval_id: MutPrimFuncVal,
    pub eval_literal: MutPrimFuncVal,
    pub eval_ref: MutPrimFuncVal,
    pub eval_eval: MutPrimFuncVal,
}

pub trait Prelude {
    fn extend(&self, memo: &mut Memo);
}

impl CorePrelude {
    pub fn new(lib: &CoreLib) -> Self {
        Self {
            not: lib.bit.not.clone(),
            and: lib.bit.and.clone(),
            or: lib.bit.or.clone(),
            xor: lib.bit.xor.clone(),
            imply: lib.bit.imply.clone(),

            add: lib.int.add.clone(),
            subtract: lib.int.subtract.clone(),
            multiply: lib.int.multiply.clone(),
            divide: lib.int.divide.clone(),
            less_than: lib.int.less_than.clone(),
            less_equal: lib.int.less_equal.clone(),
            greater_than: lib.int.greater_than.clone(),
            greater_equal: lib.int.greater_equal.clone(),

            call: lib.call.new.clone(),

            exist: lib.cfg.exist.clone(),
            import: lib.cfg.import.clone(),
            export: lib.cfg.export.clone(),
            with: lib.cfg.with.clone(),

            move_: lib.memo.remove.clone(),

            function: lib.func.new.clone(),

            get: lib.ctx.get.clone(),
            set: lib.ctx.set.clone(),
            assign: lib.ctx.assign.clone(),
            which: lib.ctx.which.clone(),

            do_: lib.ctrl.do_.clone(),
            test: lib.ctrl.test.clone(),
            switch: lib.ctrl.switch.clone(),
            match_: lib.ctrl.match_.clone(),
            loop_: lib.ctrl.loop_.clone(),
            iterate: lib.ctrl.iterate.clone(),

            type_: lib.value.type_.clone(),
            equal: lib.value.equal.clone(),

            eval: lib.lang.eval.clone(),

            form_id: form_id(),
            form_literal: form_literal(),
            form_ref: form_ref(),
            form_eval: form_eval(),
            eval_literal: eval_literal(),
            eval_id: eval_id(),
            eval_ref: eval_ref(),
            eval_eval: eval_eval(),
        }
    }
}

impl Prelude for CorePrelude {
    fn extend(&self, memo: &mut Memo) {
        memo_put_func(memo, "not", &self.not);
        memo_put_func(memo, "and", &self.and);
        memo_put_func(memo, "or", &self.or);
        memo_put_func(memo, "xor", &self.xor);
        memo_put_func(memo, "imply", &self.imply);

        memo_put_func(memo, "+", &self.add);
        memo_put_func(memo, "-", &self.subtract);
        memo_put_func(memo, "*", &self.multiply);
        memo_put_func(memo, "/", &self.divide);
        memo_put_func(memo, "<", &self.less_than);
        memo_put_func(memo, "<=", &self.less_equal);
        memo_put_func(memo, ">", &self.greater_than);
        memo_put_func(memo, ">=", &self.greater_equal);

        memo_put_func(memo, "call", &self.call);

        memo_put_func(memo, "exist", &self.exist);
        memo_put_func(memo, "import", &self.import);
        memo_put_func(memo, "export", &self.export);
        memo_put_func(memo, "with", &self.with);

        memo_put_func(memo, "move", &self.move_);

        memo_put_func(memo, "function", &self.function);

        memo_put_func(memo, "get", &self.get);
        memo_put_func(memo, "set", &self.set);
        memo_put_func(memo, "=", &self.assign);
        memo_put_func(memo, "which", &self.which);

        memo_put_func(memo, "do", &self.do_);
        memo_put_func(memo, "test", &self.test);
        memo_put_func(memo, "switch", &self.switch);
        memo_put_func(memo, "match", &self.match_);
        memo_put_func(memo, "loop", &self.loop_);
        memo_put_func(memo, "iterate", &self.iterate);

        memo_put_func(memo, "type", &self.type_);
        memo_put_func(memo, "==", &self.equal);

        memo_put_func(memo, "eval", &self.eval);

        memo_put_func(memo, DATA_ID, &self.form_id);
        memo_put_func(memo, DATA_LITERAL, &self.form_literal);
        memo_put_func(memo, DATA_REF, &self.form_ref);
        memo_put_func(memo, DATA_EVAL, &self.form_eval);
        memo_put_func(memo, CODE_ID, &self.eval_id);
        memo_put_func(memo, CODE_LITERAL, &self.eval_literal);
        memo_put_func(memo, CODE_REF, &self.eval_ref);
        memo_put_func(memo, CODE_EVAL, &self.eval_eval);
    }
}

pub fn memo_put_func<V: Clone + Into<FuncVal>>(memo: &mut Memo, name: &'static str, val: &V) {
    let name = Symbol::from_str_unchecked(name);
    let v = memo.put(name, Val::Func(val.clone().into()), Contract::None);
    assert!(matches!(v, Ok(None)), "names of preludes should be unique");
}

pub fn prelude_repr<T: Prelude>(t: T) -> Memo {
    let mut memo = Memo::default();
    t.extend(&mut memo);
    memo
}

pub fn form_id() -> FreePrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Id, CallPrimAdapter::Data);
    adapter_free_func(adapter)
}

pub fn form_literal() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Data);
    adapter_mut_func(adapter)
}

pub fn form_ref() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Ref, CallPrimAdapter::Data);
    adapter_mut_func(adapter)
}

pub fn form_eval() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Eval, CallPrimAdapter::Data);
    adapter_mut_func(adapter)
}

pub fn eval_id() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Id, CallPrimAdapter::Code);
    adapter_mut_func(adapter)
}

pub fn eval_literal() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Code);
    adapter_mut_func(adapter)
}

pub fn eval_ref() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Ref, CallPrimAdapter::Code);
    adapter_mut_func(adapter)
}

pub fn eval_eval() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Eval, CallPrimAdapter::Code);
    adapter_mut_func(adapter)
}
