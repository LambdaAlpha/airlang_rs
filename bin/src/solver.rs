use airlang::semantics::val::FuncVal;
use airlang_ext::solver::std_solver;

pub fn bin_solver() -> FuncVal {
    std_solver()
}
