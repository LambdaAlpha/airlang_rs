use crate::semantics::val::Val;
use crate::trait_::derive::derive_debug;
use crate::trait_::derive::derive_display;
use crate::type_::Call;
use crate::type_::wrap::box_wrap;

box_wrap!(pub CallVal(Call<Val, Val>));

derive_debug!(CallVal(Call<Val, Val>));

derive_display!(CallVal(Call<Val, Val>));
