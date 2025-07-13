use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::wrap::box_wrap;

box_wrap!(pub CallVal(Call<Val, Val, Val>));
