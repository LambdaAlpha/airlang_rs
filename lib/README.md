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
_("A demo of implementing a C-style for function")

_ do [
    .c_for set _ function {
        code : (.ctx : .args) : _ form _ do [
            [..init, ..condition, ..next, ..body] = .args,
            .ctx which _ eval _ form _ do [
                .init,
                .condition loop [
                    .body,
                    .next,
                ],
            ],
        ],
        raw_input : true,
    },
    _ c_for [[.i set 1, .sum set 0], i <= 10, .i set i + 1, .sum set sum + i],
    sum
]
```
