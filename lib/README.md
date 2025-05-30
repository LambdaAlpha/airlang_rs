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
_("A demo of implementing a C-like for function")

; do [
    c_for = ; function {
        context_access : mutable,
        forward_mode : id,
        code : (.ctx : .args) : ; do [
            [.init, .condition, .next, .body] = .args,
            .ctx | ; do [
                .init,
                .condition loop [
                    .body,
                    .next,
                ],
            ],
        ],
    },
    ; c_for [[i = 1, sum = 0], i <= 10, i = i + 1, sum = sum + i],
    sum
]
```
