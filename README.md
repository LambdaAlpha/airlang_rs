# The Air Programming Language

It is designed to be a universal, scalable and optimal programming language for abstraction-optimization and problem-solving.

It is an experimental proof-of-concept project and is still in the very early stages of development.

## Goals

- All reasonable abstractions and problems are expressible
- Provide a universal, scalable and optimal framework for abstraction-optimization and problem-solving

## Design

- Define `abstraction` and `problem` in theoretical computer science
- Optimize abstractions and solve problems using computability, computational complexity and reverse computation theories

## Demo

```air
"A demo of implementing a C-like for function" ! do ; [
    c_for = function ; {
        context_access : mutable,
        call_mode : id,
        call : ctx : args -> do ; [
            [.init, .condition, .next, .body] = .args,
            .ctx | do ; [
                .init,
                .condition loop [
                    .body,
                    .next,
                ],
            ],
        ],
    },
    c_for [[i = 1, sum = 0], i <= 10, i = i + 1, sum = sum + i],
    sum
]
```

## Installation

```bash
cargo install airlang_bin
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
