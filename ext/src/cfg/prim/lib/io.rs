use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;
use std::rc::Rc;

use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::extend;
use airlang::cfg::extend_func;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_CELL;
use airlang::semantics::ctx::DynCtx;
use airlang::semantics::func::CtxFreeFunc;
use airlang::semantics::func::MutFunc;
use airlang::semantics::val::DynVal;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::semantics::val::Value;
use airlang::type_::Key;
use airlang::type_::Pair;
use const_format::concatcp;

use crate::utils::memory::leak_const;

// todo design
#[derive(Copy, Clone)]
pub struct IoLib {
    pub standard_input: &'static dyn DynVal,
    pub standard_output: &'static dyn DynVal,
    pub standard_error: &'static dyn DynVal,
    pub read_line: PrimFuncVal,
    pub print: PrimFuncVal,
    pub print_line: PrimFuncVal,
    pub flush: PrimFuncVal,
}

const IO: &str = "io";

pub const STANDARD_INPUT: &str = concatcp!(PREFIX_CELL, IO, ".standard_input");
pub const STANDARD_OUTPUT: &str = concatcp!(PREFIX_CELL, IO, ".standard_output");
pub const STANDARD_ERROR: &str = concatcp!(PREFIX_CELL, IO, ".standard_error");
pub const READ_LINE: &str = concatcp!(PREFIX_CELL, IO, ".read_line");
pub const PRINT: &str = concatcp!(PREFIX_CELL, IO, ".print");
pub const PRINT_LINE: &str = concatcp!(PREFIX_CELL, IO, ".print_line");
pub const FLUSH: &str = concatcp!(PREFIX_CELL, IO, ".flush");

pub const TYPE_INPUT: &str = concatcp!(PREFIX_CELL, IO, ".input");
pub const TYPE_OUTPUT: &str = concatcp!(PREFIX_CELL, IO, ".output");

impl Default for IoLib {
    fn default() -> Self {
        Self {
            standard_input: leak_const(Input(Rc::new(RefCell::new(stdin())))),
            standard_output: leak_const(Output(Rc::new(RefCell::new(stdout())))),
            standard_error: leak_const(Output(Rc::new(RefCell::new(stderr())))),
            read_line: MutFunc { fn_: read_line }.build(),
            print: CtxFreeFunc { fn_: print }.build(),
            print_line: CtxFreeFunc { fn_: print_line }.build(),
            flush: CtxFreeFunc { fn_: flush }.build(),
        }
    }
}

impl CfgMod for IoLib {
    fn extend(self, cfg: &mut Cfg) {
        extend(cfg, STANDARD_INPUT, Val::Dyn(self.standard_input.dyn_clone()));
        extend(cfg, STANDARD_OUTPUT, Val::Dyn(self.standard_output.dyn_clone()));
        extend(cfg, STANDARD_ERROR, Val::Dyn(self.standard_error.dyn_clone()));
        extend_func(cfg, READ_LINE, self.read_line);
        extend_func(cfg, PRINT, self.print);
        extend_func(cfg, PRINT_LINE, self.print_line);
        extend_func(cfg, FLUSH, self.flush);
    }
}

pub struct Input(Rc<RefCell<dyn Read>>);

pub struct Output(Rc<RefCell<dyn Write>>);

impl Clone for Input {
    fn clone(&self) -> Self {
        Input(Rc::clone(&self.0))
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Input) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Input {}

impl Hash for Input {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{TYPE_INPUT}")
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl DynCtx<Val, Val> for Input {
    fn ref_(&self, _cfg: &mut Cfg, _key: Val) -> Option<&Val> {
        None
    }

    fn ref_mut(&mut self, _cfg: &mut Cfg, _key: Val) -> Option<&mut Val> {
        None
    }

    fn set(&mut self, _cfg: &mut Cfg, _key: Val, _value: Val) -> Option<()> {
        None
    }
}

impl Value for Input {
    fn type_name(&self) -> Key {
        Key::from_str_unchecked(TYPE_INPUT)
    }
}

impl Output {
    pub fn new(writer: Rc<RefCell<dyn Write>>) -> Self {
        Output(writer)
    }
}

impl Clone for Output {
    fn clone(&self) -> Self {
        Output(Rc::clone(&self.0))
    }
}

impl PartialEq for Output {
    fn eq(&self, other: &Output) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Output {}

impl Hash for Output {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{TYPE_OUTPUT}")
    }
}

impl Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{TYPE_OUTPUT}")
    }
}

impl DynCtx<Val, Val> for Output {
    fn ref_(&self, _cfg: &mut Cfg, _key: Val) -> Option<&Val> {
        None
    }

    fn ref_mut(&mut self, _cfg: &mut Cfg, _key: Val) -> Option<&mut Val> {
        None
    }

    fn set(&mut self, _cfg: &mut Cfg, _key: Val, _value: Val) -> Option<()> {
        None
    }
}

impl Value for Output {
    fn type_name(&self) -> Key {
        Key::from_str_unchecked(TYPE_OUTPUT)
    }
}

pub fn read_line(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Text(t) = ctx else {
        return bug!(cfg, "{READ_LINE}: expected context to be a text, but got {ctx}");
    };
    let Val::Dyn(mut val) = input else {
        return bug!(cfg, "{READ_LINE}: expected input to be a {TYPE_INPUT}, but got {input}");
    };
    let Some(val) = (&mut *val as &mut dyn Any).downcast_mut::<Input>() else {
        return bug!(cfg, "{READ_LINE}: expected input to be a {TYPE_INPUT}, but got {val}");
    };
    let Ok(mut ref_) = val.0.try_borrow_mut() else {
        return bug!(cfg, "{READ_LINE}: {TYPE_INPUT} should be available");
    };
    let mut buf_reader = BufReader::new(&mut *ref_);
    let _ = buf_reader.read_line(t);
    Val::default()
}

pub fn print(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{PRINT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Dyn(mut val) = pair.left else {
        return bug!(cfg, "{PRINT}: expected input to be a {TYPE_OUTPUT}, but got {}", pair.left);
    };
    let Some(val) = (&mut *val as &mut dyn Any).downcast_mut::<Output>() else {
        return bug!(cfg, "{PRINT}: expected input to be a {TYPE_OUTPUT}, but got {val}");
    };
    let Ok(mut ref_) = val.0.try_borrow_mut() else {
        return bug!(cfg, "{PRINT}: {TYPE_OUTPUT} should be available");
    };
    let Val::Text(t) = pair.right else {
        return bug!(cfg, "{PRINT}: expected input to be a text, but got {}", pair.right);
    };
    let _ = write!(ref_, "{}", &**t);
    Val::default()
}

pub fn print_line(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{PRINT_LINE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Dyn(mut val) = pair.left else {
        return bug!(cfg, "{PRINT_LINE}: expected input to be a {TYPE_OUTPUT}, but got {}", pair.left);
    };
    let Some(val) = (&mut *val as &mut dyn Any).downcast_mut::<Output>() else {
        return bug!(cfg, "{PRINT_LINE}: expected input to be a {TYPE_OUTPUT}, but got {val}");
    };
    let Ok(mut ref_) = val.0.try_borrow_mut() else {
        return bug!(cfg, "{PRINT_LINE}: {TYPE_OUTPUT} should be available");
    };
    let Val::Text(t) = pair.right else {
        return bug!(cfg, "{PRINT_LINE}: expected input to be a text, but got {}", pair.right);
    };
    let _ = writeln!(ref_, "{}", &**t);
    Val::default()
}

pub fn flush(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Dyn(mut val) = input else {
        return bug!(cfg, "{FLUSH}: expected input to be a {TYPE_OUTPUT}, but got {input}");
    };
    let Some(val) = (&mut *val as &mut dyn Any).downcast_mut::<Output>() else {
        return bug!(cfg, "{FLUSH}: expected input to be a {TYPE_OUTPUT}, but got {val}");
    };
    let Ok(mut ref_) = val.0.try_borrow_mut() else {
        return bug!(cfg, "{FLUSH}: {TYPE_OUTPUT} should be available");
    };
    let _ = ref_.flush();
    Val::default()
}
