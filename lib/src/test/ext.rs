use std::rc::Rc;

use crate::AirExt;
use crate::FreeStaticFn;
use crate::FreeStaticPrimFunc;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::FuncVal;
use crate::Prelude;
use crate::PreludeCtx;
use crate::Symbol;
use crate::Type;
use crate::TypeMeta;
use crate::Val;

pub(super) struct TestAirExt;

impl Prelude for TestAirExt {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        let v_ext_name = Symbol::from_str("v_ext");
        ctx.put(v_ext_name, Val::Ext(Box::new(VExt)));

        let func_ext_name = Symbol::from_str("func_ext");
        let func = FreeStaticPrimFunc::new_extension(
            func_ext_name.clone(),
            Rc::new(FuncExt),
            FuncMode::default(),
        );
        let func = Val::Func(FuncVal::FreeStaticPrim(FreeStaticPrimFuncVal::from(func)));
        ctx.put(func_ext_name, func);
    }
}

impl TypeMeta for TestAirExt {
    fn arbitrary(&self) -> Val {
        Val::Ext(Box::new(VExt))
    }

    fn arbitrary_type(&self, type1: Symbol) -> Val {
        match &*type1 {
            "v_ext" => Val::Ext(Box::new(VExt)),
            _ => Val::default(),
        }
    }
}

impl AirExt for TestAirExt {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VExt;

impl Type for VExt {
    fn type_name(&self) -> Symbol {
        Symbol::from_str("v_ext")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FuncExt;

impl FreeStaticFn for FuncExt {
    fn call(&self, input: Val) -> Val {
        input
    }
}
