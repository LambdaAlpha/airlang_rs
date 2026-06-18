use crate::cfg::prim::lib::BasePrimLib;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

#[derive(Copy, Clone)]
pub struct BasePrimPrelude {
    pub not: PrimFuncVal,
    pub and: PrimFuncVal,
    pub or: PrimFuncVal,
    pub xor: PrimFuncVal,
    pub imply: PrimFuncVal,

    pub int_add: PrimFuncVal,
    pub int_subtract: PrimFuncVal,
    pub int_multiply: PrimFuncVal,
    pub int_divide: PrimFuncVal,
    pub int_less: PrimFuncVal,
    pub int_less_equal: PrimFuncVal,
    pub int_greater: PrimFuncVal,
    pub int_greater_equal: PrimFuncVal,
    pub int_less_greater: PrimFuncVal,

    pub decimal_add: PrimFuncVal,
    pub decimal_subtract: PrimFuncVal,
    pub decimal_multiply: PrimFuncVal,
    pub decimal_divide: PrimFuncVal,
    pub decimal_less: PrimFuncVal,
    pub decimal_less_equal: PrimFuncVal,
    pub decimal_greater: PrimFuncVal,
    pub decimal_greater_equal: PrimFuncVal,
    pub decimal_less_greater: PrimFuncVal,

    pub quote: PrimFuncVal,

    pub call: PrimFuncVal,

    pub solve: PrimFuncVal,

    pub fact: PrimFuncVal,

    pub exist: PrimFuncVal,
    pub import: PrimFuncVal,
    pub export: PrimFuncVal,
    pub with: PrimFuncVal,

    pub function: PrimFuncVal,

    pub get: PrimFuncVal,
    pub take: PrimFuncVal,
    pub set: PrimFuncVal,
    pub is: PrimFuncVal,
    pub let_: PrimFuncVal,

    pub do_: PrimFuncVal,
    pub then: PrimFuncVal,
    pub branch: PrimFuncVal,
    pub match_: PrimFuncVal,
    pub loop_: PrimFuncVal,
    pub each: PrimFuncVal,

    pub equal: PrimFuncVal,

    pub abort: PrimFuncVal,
    pub assert: PrimFuncVal,

    pub eval: PrimFuncVal,
}

pub trait Prelude {
    fn extend(&self, map: &mut Map<Key, Val>);
}

impl BasePrimPrelude {
    pub fn new(lib: &BasePrimLib) -> Self {
        Self {
            not: lib.bit.not,
            and: lib.bit.and,
            or: lib.bit.or,
            xor: lib.bit.xor,
            imply: lib.bit.imply,

            int_add: lib.int.add,
            int_subtract: lib.int.subtract,
            int_multiply: lib.int.multiply,
            int_divide: lib.int.divide,
            int_less: lib.int.less,
            int_less_equal: lib.int.less_equal,
            int_greater: lib.int.greater,
            int_greater_equal: lib.int.greater_equal,
            int_less_greater: lib.int.less_greater,

            decimal_add: lib.decimal.add,
            decimal_subtract: lib.decimal.subtract,
            decimal_multiply: lib.decimal.multiply,
            decimal_divide: lib.decimal.divide,
            decimal_less: lib.decimal.less,
            decimal_less_equal: lib.decimal.less_equal,
            decimal_greater: lib.decimal.greater,
            decimal_greater_equal: lib.decimal.greater_equal,
            decimal_less_greater: lib.decimal.less_greater,

            quote: lib.quote.make,

            call: lib.call.make,

            solve: lib.solve.make,

            fact: lib.fact.make,

            exist: lib.cfg.exist,
            import: lib.cfg.import,
            export: lib.cfg.export,
            with: lib.cfg.with,

            function: lib.func.make,

            get: lib.ctx.get,
            take: lib.ctx.take,
            set: lib.ctx.set,
            is: lib.ctx.is,
            let_: lib.ctx.let_,

            do_: lib.ctrl.do_,
            then: lib.ctrl.then,
            branch: lib.ctrl.branch,
            match_: lib.ctrl.match_,
            loop_: lib.ctrl.loop_,
            each: lib.ctrl.each,

            equal: lib.value.equal,

            abort: lib.error.abort,
            assert: lib.error.assert,

            eval: lib.lang.semantics_eval,
        }
    }
}

impl Prelude for BasePrimPrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        map_put_func(map, "not", self.not);
        map_put_func(map, "and", self.and);
        map_put_func(map, "or", self.or);
        map_put_func(map, "xor", self.xor);
        map_put_func(map, "imply", self.imply);

        map_put_func(map, "+", self.int_add);
        map_put_func(map, "-", self.int_subtract);
        map_put_func(map, "*", self.int_multiply);
        map_put_func(map, "/", self.int_divide);
        map_put_func(map, "<", self.int_less);
        map_put_func(map, "<=", self.int_less_equal);
        map_put_func(map, ">", self.int_greater);
        map_put_func(map, ">=", self.int_greater_equal);
        map_put_func(map, "<>", self.int_less_greater);

        map_put_func(map, "+.", self.decimal_add);
        map_put_func(map, "-.", self.decimal_subtract);
        map_put_func(map, "*.", self.decimal_multiply);
        map_put_func(map, "/.", self.decimal_divide);
        map_put_func(map, "<.", self.decimal_less);
        map_put_func(map, "<=.", self.decimal_less_equal);
        map_put_func(map, ">.", self.decimal_greater);
        map_put_func(map, ">=.", self.decimal_greater_equal);
        map_put_func(map, "<>.", self.decimal_less_greater);

        map_put_func(map, "quote", self.quote);

        map_put_func(map, "call", self.call);

        map_put_func(map, "solve", self.solve);

        map_put_func(map, "fact", self.fact);

        map_put_func(map, "exist", self.exist);
        map_put_func(map, "import", self.import);
        map_put_func(map, "export", self.export);
        map_put_func(map, "with", self.with);

        map_put_func(map, "function", self.function);

        map_put_func(map, "get", self.get);
        map_put_func(map, "take", self.take);
        map_put_func(map, "set", self.set);
        map_put_func(map, "is", self.is);
        map_put_func(map, "let", self.let_);

        map_put_func(map, "do", self.do_);
        map_put_func(map, "then", self.then);
        map_put_func(map, "branch", self.branch);
        map_put_func(map, "match", self.match_);
        map_put_func(map, "loop", self.loop_);
        map_put_func(map, "each", self.each);

        map_put_func(map, "=", self.equal);

        map_put_func(map, "abort", self.abort);
        map_put_func(map, "assert", self.assert);

        map_put_func(map, "eval", self.eval);
    }
}

pub fn map_put_func(map: &mut Map<Key, Val>, name: &'static str, val: PrimFuncVal) {
    let name = Key::from_str_unchecked(name);
    let v = map.insert(name, Val::Func(val.into()));
    assert!(v.is_none(), "names of preludes should be unique");
}

pub fn prelude_repr<T: Prelude>(t: T) -> Map<Key, Val> {
    let mut map = Map::default();
    t.extend(&mut map);
    map
}
