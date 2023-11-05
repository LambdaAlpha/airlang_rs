# The Air Programming Language

The [Air](https://github.com/LambdaAlpha/airlang) programming language is carefully designed to solve programming problems once and for all. It is a delightful surprise for programming language enthusiasts.

## Goals

The Air language seeks to solve programming problems once and for all.

- It can express any describable information, such as requirements and implementations, problems and solutions, propositions and proofs.
- It can provide any information about the language and the program itself.
- It can implement any theoretically possible information processing requirement, such as implementing requirements, answering questions, and proving propositions.
- It can use information about the language and the program itself to perform property proofs and performance optimizations, achieving the best properties and optimal performance.
- It provides stable syntax and semantics, allowing users to learn the programming language once and for all.

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
"
Demonstration of the gcd algorithm\n
\n
- The syntax for comments `a @ b` is not special. It takes a pair of values and returns the second value.\n
- There are very few keywords\n
\  - `@` and `:` are soft keywords, representing comments and pairs respectively.\s
     The symbols themselves can be represented by `'@` and `':` respectively.\n
\  - `context`, `function`, `while`, `move`, `map`, `body` are all ordinary symbols.\n
- There are no operators, `;`, `=`, `<>`, `%` are all ordinary symbols.\n
- The symbols `;`, `=`, `context`, `function`, `while`, `<>`, `%`, `move` and `gcd` all refer to ordinary functions.\n
"
@
(; [
    ctx = (context {map : {
        : ;,
        : =,
        : while,
        : <>,
        : %,
    }}),
    gcd = (function {
        body : (; [
            (x : y) = input,
            while [
                y <> 0,
                ; [
                    z = y,
                    y = (x % y),
                    x = z,
                ],
            ],
            x
        ]),
        context : (move ctx),
    }),
    42 gcd 24
])
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
