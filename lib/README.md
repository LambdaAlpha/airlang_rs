# The Air Programming Language

The [Air](https://github.com/LambdaAlpha/airlang) programming language is carefully designed to solve programming problems once and for all.

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
"Demonstration of the gcd algorithm" @ ; ! [
    ctx = . context {
        : ;,
        : =,
        : while,
        : <>,
        : %,
    },
    gcd = function ! {
        body : ; ! [
            (x : y) = the_input,
            (y <> 0) while [
                z = y,
                y = x % y,
                x = z,
            ],
            x
        ],
        prelude : move ! ctx,
    },
    42 gcd 24
]
```
