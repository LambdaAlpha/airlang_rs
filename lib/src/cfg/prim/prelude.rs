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

    pub move_: PrimFuncVal,

    pub quote: PrimFuncVal,

    pub call: PrimFuncVal,

    pub solve: PrimFuncVal,

    pub exist: PrimFuncVal,
    pub import: PrimFuncVal,
    pub export: PrimFuncVal,
    pub with: PrimFuncVal,

    pub function: PrimFuncVal,

    pub get: PrimFuncVal,
    pub set: PrimFuncVal,
    pub represent: PrimFuncVal,
    pub which: PrimFuncVal,

    pub do_: PrimFuncVal,
    pub test: PrimFuncVal,
    pub switch: PrimFuncVal,
    pub match_: PrimFuncVal,
    pub loop_: PrimFuncVal,
    pub iterate: PrimFuncVal,

    pub get_type: PrimFuncVal,
    pub equal: PrimFuncVal,

    pub abort: PrimFuncVal,
    pub assert: PrimFuncVal,

    pub fact: PrimFuncVal,

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

            move_: lib.map.move_,

            quote: lib.quote.make,

            call: lib.call.make,

            solve: lib.solve.make,

            exist: lib.cfg.exist,
            import: lib.cfg.import,
            export: lib.cfg.export,
            with: lib.cfg.with,

            function: lib.func.make,

            get: lib.ctx.get,
            set: lib.ctx.set,
            represent: lib.ctx.represent,
            which: lib.ctx.which,

            do_: lib.ctrl.do_,
            test: lib.ctrl.test,
            switch: lib.ctrl.switch,
            match_: lib.ctrl.match_,
            loop_: lib.ctrl.loop_,
            iterate: lib.ctrl.iterate,

            get_type: lib.value.get_type,
            equal: lib.value.equal,

            abort: lib.error.abort,
            assert: lib.error.assert,

            fact: lib.fact.put,

            eval: lib.lang.eval,
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

        map_put_func(map, "move", self.move_);

        map_put_func(map, "quote", self.quote);

        map_put_func(map, "call", self.call);

        map_put_func(map, "solve", self.solve);

        map_put_func(map, "exist", self.exist);
        map_put_func(map, "import", self.import);
        map_put_func(map, "export", self.export);
        map_put_func(map, "with", self.with);

        map_put_func(map, "function", self.function);

        map_put_func(map, "get", self.get);
        map_put_func(map, "set", self.set);
        map_put_func(map, "represent", self.represent);
        map_put_func(map, "which", self.which);

        map_put_func(map, "do", self.do_);
        map_put_func(map, "test", self.test);
        map_put_func(map, "switch", self.switch);
        map_put_func(map, "match", self.match_);
        map_put_func(map, "loop", self.loop_);
        map_put_func(map, "iterate", self.iterate);

        map_put_func(map, "get_type", self.get_type);
        map_put_func(map, "=", self.equal);

        map_put_func(map, "abort", self.abort);
        map_put_func(map, "assert", self.assert);

        map_put_func(map, "fact", self.fact);

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
