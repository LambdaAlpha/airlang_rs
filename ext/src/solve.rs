use airlang::semantics::val::FuncVal;
use airlang::solve::core_solver;

pub fn std_solver() -> FuncVal {
    core_solver()
}
