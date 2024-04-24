use airlang::{
    initial_ctx,
    interpret_mutable,
    parse,
    MutableCtx,
    Val,
};

pub fn main() -> Val {
    load(include_str!("../main/main.air"))
}

pub fn ext() -> Val {
    load(include_str!("../ext/ext.air"))
}

pub(crate) fn load(src: &str) -> Val {
    let Ok(val) = parse(src) else { unreachable!() };
    let mut ctx = initial_ctx();
    interpret_mutable(MutableCtx::new(&mut ctx), val)
}
