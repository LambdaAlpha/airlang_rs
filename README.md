# Air Programming Language

## Design Goals

- **Minimalist**  
  A language is a consensus among programmers. The simpler the language, the stronger the consensus and the easier the code is to understand. Therefore, we strive to avoid unnecessary complexity. Based on this principle, we do not build in features such as modules, control flow, assignment, pattern matching, or type constructors into the language core.

- **Universal**  
  The broader a language's applicable scenarios, the higher the return on investment in learning it, and the better the interoperability between projects. Therefore, we aim to make the language adaptable to various goals and resource scales. Based on this principle, we provide users with the ability to manage context, configuration, and resources.

## Language Features

### Minimalist Syntax

Air's syntax is extremely concise. It only includes comments and 11 data types, with no semantic-specific syntax for functions, types, modules, etc. Its rules are very simple, using prefixes to avoid ambiguity, and it has only 5 keywords (`_`, `.`, `:`, `true`, `false`). This makes it highly suitable as a configuration language or data interchange format.

**comment**

`_(t1 t2 ... tn)`

```air
_("comment")
[1, _(2, 3,) 4]
{a : _(1, b :) 2}
```

**unit**

```air
.
```

**bit**

```air
true
false
```

**key**

```air
key

>=

a.b.c

'[0, 1, 2]'

' abcdefghijklmnopqrstuvwxyz
| ABCDEFGHIJKLMNOPQRSTUVWXYZ
|(()[]{}<>\|/'"`^~-+=*_.,:;!?@#$%&
|)0123456789'
```

**text**

```air
"🜁^u(1F701)"

"- a^r^n^t- a.1^r^n^t- a.2"

 "- a
+   - a.1
+   - a.2"
```

**integer**

```air
123
0-123
integer(-123)
0X7f
0-B1110
```

**number**

```air
0.1
0-0.1
1.0E-1
number(-0.1)
```

**byte**

```air
byte(B00001111)
byte(X00ffff)
```

**pair**

`first : second`

```air
a : 1
a : b : c
```

**list**

`[v1, v2, ..., vn]`

```air
[0, 1, 2]
[., false, 0, '',]
```

**map**

`{k1 : v1, k2 : v2, ..., kn : vn}`

```air
{a : 1, b : 2, c : 3}
{a : 1, b : true, c : ' ',}
{a, b, c}
```

**call**

- `_ function input`
- `input function _`
- `first function second`

```air
_ not true
1 + 1
a and b or c
```

### Minimalist Semantics

Air's evaluation rules are very concise, consisting of only four rules.

First, the evaluation rules for keys are as follows:

1. `_a` ➔ `_a`
2. `.a` ➔ `a`
3. `:a` or `a` ➔ `v`, where `v` is the value bound to key `a` in the context

Second, the evaluation rule for calls is `_ f i` ➔ `o`, with the following steps:

1. `eval(f)` ➔ `vf`
2. `if vf.raw_input then i else eval(i)` ➔ `vi`
3. `vf(vi)` ➔ `o`

Third, the evaluation rules for pairs, lists, and maps are as follows:

- `v1 : v2` ➔ `eval(v1) : eval(v2)`
- `[v1, v2, ..., vn]` ➔ `[eval(v1), eval(v2), ..., eval(vn)]`
- `{k1 : v1, k2 : v2, ..., kn : vn}` ➔ `{k1 : eval(v1), k2 : eval(v2), kn : eval(vn)}`

Fourth, the evaluation rule for other values is `v` ➔ `v`.

### Context

The context is the local information environment during execution. In core semantics, the context can be accessed via keys, and functions also support sensing or updating the context. Variables in the context can be read via the `get` function, updated via the `set` function, or specified via the `which` function. Based on this capability of functions, we implement various control flow functions, including sequential execution `do`, conditional execution `test`, pattern matching `match`, loops `loop`, iteration `iterate`, etc. The most commonly used and essential core functions are provided in the initial context.

```air
_ do [
    .sum set 0,
    100 iterate i : [
        .sum set sum + i
    ],
    sum
]
```

### Configuration

Configuration is the global information environment during execution. Through mechanisms like append-only and scoped override, it balances flexibility and predictability. Configuration items can be imported via the `import` function, exported via the `export` function, or locally overridden via the `with` function. We will implement features like module management, testing frameworks, and exception handling based on the configuration mechanism, and provide native functions and standard libraries in the initial configuration.

```air
_ do [
    .push set _ import _list.push,
    _list.add export push,
    _list.append export push,
]
```

### Resources

Resources are scarce, consumable entities required during execution, with the most critical being execution time and storage space. Available execution steps can be read via `available_steps`, measured via `measure_steps`, or limited via `limit_steps`. We will gradually build a resource management framework around these basic capabilities to provide essential foundational support for the development of resource-sensitive applications such as artificial intelligence.

```air
_ do [
    .limit_steps set _ import _resource.limit_steps,
    100 limit_steps _ data true loop []
]
```

## Roadmap

1. **Enhance Language Expressiveness**  
   Focus on expanding core expressiveness to lay the foundation for subsequent capabilities.

2. **Introduce Abstract Semantics and Program Optimization Framework**  
   Introduce an abstract semantics model based on "concrete value + abstract constraint," enabling optimization of values within the same class while maintaining semantic equivalence, and build a general program optimization framework based on this.

3. **Develop Intelligent Optimization Algorithms**  
   Develop automated, intelligent optimization algorithms based on abstract semantics to systematically optimize program resource usage.

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
