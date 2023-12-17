#![deny(
    bad_style,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_interfaces,
    private_bounds,
    unconditional_recursion,
    while_true
)]
#![cfg_attr(
    not(debug_assertions),
    deny(
        dead_code,
        unused,
        unused_allocation,
        unused_comparisons,
        unused_parens,
        clippy::needless_return,
        clippy::semicolon_if_nothing_returned,
    )
)]
#![feature(trait_alias)]

use {
    crate::prelude::{
        AllPrelude,
        Prelude,
        PreludeMap,
    },
    airlang::{
        set_extension,
        Symbol,
    },
    airlang_ext::{
        AirExt,
        ExtFunc,
    },
};

fn main() -> std::io::Result<()> {
    let all_prelude = AllPrelude::default();
    let mut map = ReplPreludeMap(AirExt::default());
    all_prelude.put(&mut map);
    set_extension(Box::new(map.0));
    repl::repl()
}

struct ReplPreludeMap(AirExt);

impl PreludeMap for ReplPreludeMap {
    fn put(&mut self, name: Symbol, func: ExtFunc) {
        self.0.add_func(name, func);
    }
}

mod repl;

mod prelude;
