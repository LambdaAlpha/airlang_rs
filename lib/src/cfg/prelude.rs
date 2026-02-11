use crate::cfg::lib::CoreLib;
use crate::semantics::val::FuncVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

#[derive(Clone)]
pub struct CorePrelude {
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

    pub call: PrimFuncVal,

    pub move_: PrimFuncVal,

    pub exist: PrimFuncVal,
    pub import: PrimFuncVal,
    pub export: PrimFuncVal,
    pub with: PrimFuncVal,

    pub function: PrimFuncVal,
    pub apply: PrimFuncVal,

    pub get: PrimFuncVal,
    pub set: PrimFuncVal,
    pub form: PrimFuncVal,
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

    pub data: PrimFuncVal,
    pub id: PrimFuncVal,
    pub code: PrimFuncVal,
    pub eval: PrimFuncVal,
}

pub trait Prelude {
    fn extend(&self, map: &mut Map<Key, Val>);
}

impl CorePrelude {
    pub fn new(lib: &CoreLib) -> Self {
        Self {
            not: lib.bit.not.clone(),
            and: lib.bit.and.clone(),
            or: lib.bit.or.clone(),
            xor: lib.bit.xor.clone(),
            imply: lib.bit.imply.clone(),

            int_add: lib.int.add.clone(),
            int_subtract: lib.int.subtract.clone(),
            int_multiply: lib.int.multiply.clone(),
            int_divide: lib.int.divide.clone(),
            int_less: lib.int.less.clone(),
            int_less_equal: lib.int.less_equal.clone(),
            int_greater: lib.int.greater.clone(),
            int_greater_equal: lib.int.greater_equal.clone(),
            int_less_greater: lib.int.less_greater.clone(),

            decimal_add: lib.decimal.add.clone(),
            decimal_subtract: lib.decimal.subtract.clone(),
            decimal_multiply: lib.decimal.multiply.clone(),
            decimal_divide: lib.decimal.divide.clone(),
            decimal_less: lib.decimal.less.clone(),
            decimal_less_equal: lib.decimal.less_equal.clone(),
            decimal_greater: lib.decimal.greater.clone(),
            decimal_greater_equal: lib.decimal.greater_equal.clone(),
            decimal_less_greater: lib.decimal.less_greater.clone(),

            call: lib.call.new.clone(),

            move_: lib.map.move_.clone(),

            exist: lib.cfg.exist.clone(),
            import: lib.cfg.import.clone(),
            export: lib.cfg.export.clone(),
            with: lib.cfg.with.clone(),

            function: lib.func.new.clone(),
            apply: lib.func.apply.clone(),

            get: lib.ctx.get.clone(),
            set: lib.ctx.set.clone(),
            form: lib.ctx.form.clone(),
            represent: lib.ctx.represent.clone(),
            which: lib.ctx.which.clone(),

            do_: lib.ctrl.do_.clone(),
            test: lib.ctrl.test.clone(),
            switch: lib.ctrl.switch.clone(),
            match_: lib.ctrl.match_.clone(),
            loop_: lib.ctrl.loop_.clone(),
            iterate: lib.ctrl.iterate.clone(),

            get_type: lib.value.get_type.clone(),
            equal: lib.value.equal.clone(),

            abort: lib.error.abort.clone(),
            assert: lib.error.assert.clone(),

            data: lib.lang.data.clone(),
            id: lib.lang.id.clone(),
            code: lib.lang.code.clone(),
            eval: lib.lang.eval.clone(),
        }
    }
}

impl Prelude for CorePrelude {
    fn extend(&self, map: &mut Map<Key, Val>) {
        map_put_func(map, "not", &self.not);
        map_put_func(map, "and", &self.and);
        map_put_func(map, "or", &self.or);
        map_put_func(map, "xor", &self.xor);
        map_put_func(map, "imply", &self.imply);

        map_put_func(map, "+", &self.int_add);
        map_put_func(map, "-", &self.int_subtract);
        map_put_func(map, "*", &self.int_multiply);
        map_put_func(map, "/", &self.int_divide);
        map_put_func(map, "<", &self.int_less);
        map_put_func(map, "<=", &self.int_less_equal);
        map_put_func(map, ">", &self.int_greater);
        map_put_func(map, ">=", &self.int_greater_equal);
        map_put_func(map, "<>", &self.int_less_greater);

        map_put_func(map, "+.", &self.decimal_add);
        map_put_func(map, "-.", &self.decimal_subtract);
        map_put_func(map, "*.", &self.decimal_multiply);
        map_put_func(map, "/.", &self.decimal_divide);
        map_put_func(map, "<.", &self.decimal_less);
        map_put_func(map, "<=.", &self.decimal_less_equal);
        map_put_func(map, ">.", &self.decimal_greater);
        map_put_func(map, ">=.", &self.decimal_greater_equal);
        map_put_func(map, "<>.", &self.decimal_less_greater);

        map_put_func(map, "call", &self.call);

        map_put_func(map, "move", &self.move_);

        map_put_func(map, "exist", &self.exist);
        map_put_func(map, "import", &self.import);
        map_put_func(map, "export", &self.export);
        map_put_func(map, "with", &self.with);

        map_put_func(map, "function", &self.function);
        map_put_func(map, "apply", &self.apply);

        map_put_func(map, "get", &self.get);
        map_put_func(map, "set", &self.set);
        map_put_func(map, "form", &self.form);
        map_put_func(map, "=", &self.represent);
        map_put_func(map, "which", &self.which);

        map_put_func(map, "do", &self.do_);
        map_put_func(map, "test", &self.test);
        map_put_func(map, "switch", &self.switch);
        map_put_func(map, "match", &self.match_);
        map_put_func(map, "loop", &self.loop_);
        map_put_func(map, "iterate", &self.iterate);

        map_put_func(map, "get_type", &self.get_type);
        map_put_func(map, "==", &self.equal);

        map_put_func(map, "abort", &self.abort);
        map_put_func(map, "assert", &self.assert);

        map_put_func(map, "data", &self.data);
        map_put_func(map, "id", &self.id);
        map_put_func(map, "code", &self.code);
        map_put_func(map, "eval", &self.eval);
    }
}

pub fn map_put_func<V: Clone + Into<FuncVal>>(
    map: &mut Map<Key, Val>, name: &'static str, val: &V,
) {
    let name = Key::from_str_unchecked(name);
    let v = map.insert(name, Val::Func(val.clone().into()));
    assert!(v.is_none(), "names of preludes should be unique");
}

pub fn prelude_repr<T: Prelude>(t: T) -> Map<Key, Val> {
    let mut map = Map::default();
    t.extend(&mut map);
    map
}
