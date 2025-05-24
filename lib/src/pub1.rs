pub use crate::bit::Bit;
pub use crate::byte::Byte;
pub use crate::call::Call;
pub use crate::ctx::Ctx;
pub use crate::ctx::CtxAccess;
pub use crate::ctx::CtxError;
pub use crate::ctx::map::Contract;
pub use crate::either::Either;
pub use crate::extension::AirExt;
pub use crate::extension::ValExt;
pub use crate::func::const_cell_comp::ConstCellCompFunc;
pub use crate::func::const_cell_prim::ConstCellFn;
pub use crate::func::const_cell_prim::ConstCellFnExt;
pub use crate::func::const_cell_prim::ConstCellPrimFunc;
pub use crate::func::const_static_comp::ConstStaticCompFunc;
pub use crate::func::const_static_prim::ConstStaticFn;
pub use crate::func::const_static_prim::ConstStaticImpl;
pub use crate::func::const_static_prim::ConstStaticPrimFunc;
pub use crate::func::free_cell_comp::FreeCellCompFunc;
pub use crate::func::free_cell_prim::FreeCellFn;
pub use crate::func::free_cell_prim::FreeCellFnExt;
pub use crate::func::free_cell_prim::FreeCellPrimFunc;
pub use crate::func::free_static_comp::FreeStaticCompFunc;
pub use crate::func::free_static_prim::FreeStaticFn;
pub use crate::func::free_static_prim::FreeStaticImpl;
pub use crate::func::free_static_prim::FreeStaticPrimFunc;
pub use crate::func::func_mode::FuncMode;
pub use crate::func::mode::ModeFunc;
pub use crate::func::mut_cell_comp::MutCellCompFunc;
pub use crate::func::mut_cell_prim::MutCellFn;
pub use crate::func::mut_cell_prim::MutCellFnExt;
pub use crate::func::mut_cell_prim::MutCellPrimFunc;
pub use crate::func::mut_static_comp::MutStaticCompFunc;
pub use crate::func::mut_static_prim::MutStaticFn;
pub use crate::func::mut_static_prim::MutStaticImpl;
pub use crate::func::mut_static_prim::MutStaticPrimFunc;
pub use crate::int::Int;
pub use crate::list::List;
pub use crate::map::Map;
pub use crate::mode::Mode;
pub use crate::mode::call::CallMode;
pub use crate::mode::comp::CompMode;
pub use crate::mode::list::ListMode;
pub use crate::mode::map::MapMode;
pub use crate::mode::pair::PairMode;
pub use crate::mode::prim::CodeMode;
pub use crate::mode::prim::DataMode;
pub use crate::mode::prim::PrimMode;
pub use crate::mode::symbol::SymbolMode;
pub use crate::number::Number;
pub use crate::pair::Pair;
pub use crate::prelude::Prelude;
pub use crate::prelude::PreludeCtx;
pub use crate::symbol::Symbol;
pub use crate::text::Text;
pub use crate::type1::Type;
pub use crate::type1::TypeMeta;
pub use crate::types::ref1::ConstRef;
pub use crate::types::ref1::DynRef;
pub use crate::unit::Unit;
pub use crate::val::Val;
pub use crate::val::byte::ByteVal;
pub use crate::val::call::CallVal;
pub use crate::val::ctx::CtxVal;
pub use crate::val::func::FuncVal;
pub use crate::val::func::const_cell_comp::ConstCellCompFuncVal;
pub use crate::val::func::const_cell_prim::ConstCellPrimFuncVal;
pub use crate::val::func::const_static_comp::ConstStaticCompFuncVal;
pub use crate::val::func::const_static_prim::ConstStaticPrimFuncVal;
pub use crate::val::func::free_cell_comp::FreeCellCompFuncVal;
pub use crate::val::func::free_cell_prim::FreeCellPrimFuncVal;
pub use crate::val::func::free_static_comp::FreeStaticCompFuncVal;
pub use crate::val::func::free_static_prim::FreeStaticPrimFuncVal;
pub use crate::val::func::mode::ModeFuncVal;
pub use crate::val::func::mut_cell_comp::MutCellCompFuncVal;
pub use crate::val::func::mut_cell_prim::MutCellPrimFuncVal;
pub use crate::val::func::mut_static_comp::MutStaticCompFuncVal;
pub use crate::val::func::mut_static_prim::MutStaticPrimFuncVal;
pub use crate::val::int::IntVal;
pub use crate::val::list::ListVal;
pub use crate::val::map::MapVal;
pub use crate::val::number::NumberVal;
pub use crate::val::pair::PairVal;
pub use crate::val::text::TextVal;

// https://github.com/rust-lang/rustfmt/issues/4070
mod __ {}

use crate::extension::set_air_ext;
use crate::syntax::ParseError;
use crate::syntax::ReprError;
use crate::syntax::generator::PRETTY_FMT;

pub fn parse(src: &str) -> Result<Val, ParseError> {
    crate::syntax::parser::parse(src)
}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    let repr = src.try_into()?;
    let repr = crate::syntax::generator::generate(repr, PRETTY_FMT);
    Ok(repr)
}

#[derive(Debug, Clone)]
pub struct AirCell {
    mode: Option<Mode>,
    ctx: Ctx,
}

impl AirCell {
    /// this method should be called before instantiating `AirCell` or calling `initial_ctx`
    pub fn set_ext(provider: Box<dyn AirExt>) {
        set_air_ext(provider);
    }

    pub fn new(mode: Option<Mode>, ctx: Ctx) -> Self {
        Self { mode, ctx }
    }

    pub fn initial_ctx() -> Ctx {
        crate::prelude::initial_ctx()
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        self.mode.mut_static_call(&mut self.ctx, input)
    }

    pub fn ctx_mut(&mut self) -> &mut Ctx {
        &mut self.ctx
    }
}

impl Default for AirCell {
    fn default() -> Self {
        Self { mode: FuncMode::default_mode(), ctx: Self::initial_ctx() }
    }
}
