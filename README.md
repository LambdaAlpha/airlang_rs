# The Air Programming Language

The Air programming language is carefully designed to solve programming problems once and for all.

It is an experimental proof-of-concept project and is still in the very early stages of development.

## Goals

The Air language seeks to solve programming problems once and for all. It should be able to

- express any describable information, such as requirements and implementations, problems and solutions, propositions and proofs.
- provide any information about the language and the program itself.
- implement any theoretically possible information processing requirement, such as implementing requirements, answering questions, and proving propositions.
- use information about the language and the program itself to perform property proofs and performance optimizations, achieving the best properties and optimal performance.
- provides stable syntax and semantics, allowing users to learn the programming language once and for all.

## Non-Goals

- No design choices are taken for granted, and language features are not copied from other languages without review.
- Suboptimal designs are not chosen to accommodate user habits.
- Solutions that only solve most but not all problems are not satisfactory.
- Impossible tasks are not attempted to be implemented.
- The language is not constantly updated to implement more requirements.

## Design

- Decouple syntax from semantics, making syntax available as a general data exchange format.
- Build a concise semantic core and provide rich initial context.
- Allow functions to access context, which means that control statements are just functions that can access context.
- Implement a universal logical framework based on computability theory, replacing type systems based on type theory.
- Implement a universal problem framework based on reverse computation theory, used to express any describable requirement or problem, replacing interface/trait systems.
- Implement a universal algorithm framework based on complexity theory, attempting to achieve artificial general intelligence.

## Demo

```Air
"A demo of implementing a C-like for loop function" @ ; ! [
    c_for = function ! {
        input_name : .args,
        context_name : .ctx,
        context_access : .mutable,
        input_mode : id,
        prelude : prelude ! .,
        body : ; ! [
            [init, condition, next, body] = args,
            ctx | form ! ; ! [
                .&init,
                .&condition while [
                    .&body,
                    .&next,
                ],
            ],
        ],
    },
    c_for [[i = 1, sum = 0], i <= 10, i = i + 1, sum = sum + i],
    sum
]
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
