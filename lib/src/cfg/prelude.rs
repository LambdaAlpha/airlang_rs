use crate::cfg::lib::CoreLib;
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
    pub apply: MutPrimFuncVal,

    pub get: ConstPrimFuncVal,
    pub set: MutPrimFuncVal,
    pub form: ConstPrimFuncVal,
    pub repr: MutPrimFuncVal,
    pub which: MutPrimFuncVal,

    pub do_: MutPrimFuncVal,
    pub test: MutPrimFuncVal,
    pub switch: MutPrimFuncVal,
    pub match_: MutPrimFuncVal,
    pub loop_: MutPrimFuncVal,
    pub iterate: MutPrimFuncVal,

    pub type_: ConstPrimFuncVal,
    pub equal: FreePrimFuncVal,

    pub data: FreePrimFuncVal,
    pub id: FreePrimFuncVal,
    pub code: MutPrimFuncVal,
    pub eval: MutPrimFuncVal,
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
            apply: lib.func.apply.clone(),

            get: lib.ctx.get.clone(),
            set: lib.ctx.set.clone(),
            form: lib.ctx.form.clone(),
            repr: lib.ctx.repr.clone(),
            which: lib.ctx.which.clone(),

            do_: lib.ctrl.do_.clone(),
            test: lib.ctrl.test.clone(),
            switch: lib.ctrl.switch.clone(),
            match_: lib.ctrl.match_.clone(),
            loop_: lib.ctrl.loop_.clone(),
            iterate: lib.ctrl.iterate.clone(),

            type_: lib.value.type_.clone(),
            equal: lib.value.equal.clone(),

            data: lib.lang.data.clone(),
            id: lib.lang.id.clone(),
            code: lib.lang.code.clone(),
            eval: lib.lang.eval.clone(),
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
        memo_put_func(memo, "apply", &self.apply);

        memo_put_func(memo, "get", &self.get);
        memo_put_func(memo, "set", &self.set);
        memo_put_func(memo, "form", &self.form);
        memo_put_func(memo, "=", &self.repr);
        memo_put_func(memo, "which", &self.which);

        memo_put_func(memo, "do", &self.do_);
        memo_put_func(memo, "test", &self.test);
        memo_put_func(memo, "switch", &self.switch);
        memo_put_func(memo, "match", &self.match_);
        memo_put_func(memo, "loop", &self.loop_);
        memo_put_func(memo, "iterate", &self.iterate);

        memo_put_func(memo, "type", &self.type_);
        memo_put_func(memo, "==", &self.equal);

        memo_put_func(memo, "data", &self.data);
        memo_put_func(memo, "id", &self.id);
        memo_put_func(memo, "code", &self.code);
        memo_put_func(memo, "eval", &self.eval);
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
