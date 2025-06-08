use crate::semantics::solver::unit_solver;
use crate::semantics::val::FuncVal;

pub fn core_solver() -> FuncVal {
    unit_solver()
}
