# Air Programming Language

## Design Goals

- **Minimalist**  
  A language is a consensus among programmers. The simpler the language, the stronger the consensus and the easier the code is to understand. Therefore, we strive to avoid unnecessary complexity. Based on this principle, we do not build in features such as modules, control flow, assignment, pattern matching, or type constructors into the language core.

- **Universal**  
  The broader a language's applicable scenarios, the higher the return on investment in learning it, and the better the interoperability between projects. Therefore, we aim to make the language adaptable to various goals and resource scales. Based on this principle, we provide users with the ability to manage context, configuration, and resources.

## Language Features

### Minimalist Syntax

Air's syntax is extremely concise. It only includes comments and 13 data types, with no semantic-specific syntax for control flows, functions, types, modules, etc. Its rules are very simple, using prefixes to avoid ambiguity, and it has only 5 keywords (`_`, `.`, `:`, `true`, `false`). This makes it highly suitable for configuration or data interchange.

**unit**

`.`

**bit**

- `true`
- `false`

**key**

- `'key'`
- `key`

```air
>=

a.b.c

'[0, 1, 2]'

'^(X3f ' ^ sp)'

'abcdefghijklmnopqrstuvwxyz
|_()[]{}<>\|/'"`^*+=-~_.,:;!?@#$%&
|^sp 0 1 2 3 4 5 6 7 8 9
|!this line is a comment
|'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
```

**text**

`"text"`

```air
"🜁: Alchemical Symbol For Air"

"^(X1f701 " ^ sp ht cr lf)"

    "
    |_()[]{}<>\|/
    | '"`^*+=-~_
    | .,:;!?
    | @#$%&
    |^X1f701 " ^ sp ht cr lf
    |!this line is a comment
    |""
```

**integer**

- `integer'0'`
- `0-1` = `integer'-1'`

```air
123
0-123
integer'-123'
0X7f
integer'X7f'
0-B1110
```

**decimal**

- `decimal'0.'`
- `0-1.` = `decimal'-1.'`

```air
12.3
0-12.3
decimal'-12.3'
0-E-12*3.456
decimal'-E-12*3.456'
```

**byte**

`byte'00'`

```air
byte'B00001111'
byte'X00ffff'
```

**cell**

- `.(v)`
- `.'key'` = `.('key')`
- `."text"` = `.("text")`
- `.[l, i, s, t]` = `.([l, i, s, t])`
- `.{a : map}` = `.({a : map})`

```air
.(true)
.('cell')
.(.[.{a : .""}])
```

**pair**

`left : right`

```air
a : 1
a : b : c
```

**list**

- `[v1, v2, ..., vn]`
- `#[v1 v2 ... vn]` = `[v1, v2, ..., vn]`

```air
[0, 1, 2]
[., false, 0, '',]
#[git commit --amend --no-edit]
```

**map**

- `{k1 : v1, k2 : v2, ... : ..., kn : vn}`
- `#{k1 v1 k2 v2 ... ... kn vn}` = `{k1 : v1, k2 : v2, ... : ..., kn : vn}`

```air
{a : 1, b : 2, c : 3}
{a : 1, b : true, c : ' ',}
{a, b, c}
#{
    select *
    from book
    where (price > 100)
    order_by title
}
```

**quote**

- `_(v)`
- `_'key'` = `_('key')`
- `_"text"` = `_("text")`
- `_[l, i, s, t]` = `_([l, i, s, t])`
- `_{a : map}` = `_({a : map})`

```air
_(true)
_('quote')
_(_[_{a : _""}])
```

**call**

- `_ function input`
- `input function _`
- `left function right` = `_ function left : right`

```air
_ not true
1 + 1
a and b or c
```

**comment**

- `!(t1 t2 ... tn)`
- `!'key'`
- `!"text"`
- `![l, i, s, t]`
- `!{a : map}`

```air
!"comment"
[1, !(2, 3,) 4]
{a : !(1, b :) 2}
```

### Minimalist Semantics

Air's evaluation rules are very concise, consisting of only five rules.

First, the evaluation rules for keys are as follows:

1. `_a` ➔ `a`
2. `.a` ➔ `.a`
3. `a` ➔ `v`, where `v` is the value bound to key `a` in the context

Second, the evaluation rule for quotes is `_(v)` ➔ `v`.

Third, the evaluation rule for calls is `_ f i ➔ f'(i')`, where `x'` denotes the result of evaluating `x` (the same applies below).

Fourth, the evaluation rules for cells, pairs, lists, and maps are as follows:

- `.(v)` ➔ `.(v')`
- `v1 : v2` ➔ `v1' : v2'`
- `[v1, v2, ..., vn]` ➔ `[v1', v2', ..., vn']`
- `{k1 : v1, k2 : v2, ..., kn : vn}` ➔ `{k1 : v1', k2 : v2', kn : vn'}`

Fifth, the evaluation rule for other values is `v` ➔ `v`.

### Context

The context is the local information environment during execution. In core semantics, the context can be accessed via keys, and functions also support sensing or updating the context. Variables in the context can be read via the `get` function, updated via the `set` function, or specified via the `which` function. Based on this capability of functions, we implement various control flow functions, including sequential execution `do`, conditional execution `test`, pattern matching `match`, loops `loop`, iteration `iterate`, etc. The most commonly used and essential core functions are provided in the initial context.

```air
_ do _[
    _sum set 0,
    100 iterate _i : _[
        _sum set sum + i
    ],
    sum
]
```

### Configuration

Configuration is the global information environment during execution. Through mechanisms like append-only and scoped override, it balances flexibility and predictability. Configuration items can be imported via the `import` function, exported via the `export` function, or locally overridden via the `with` function. We will implement features like module management, testing frameworks, and error handling based on the configuration mechanism, and provide native functions and standard libraries in the initial configuration.

```air
_ do _[
    _push set _ import .list.push,
    .list.add export push,
    .list.append export push,
]
```

### Resources

Resources are scarce, consumable entities required during execution, with the most critical being execution time and storage space. Available execution steps can be read via `get_steps`, measured via `measure_steps`, or limited via `set_steps`. We will gradually build a resource management framework around these basic capabilities to provide essential foundational support for the development of resource-sensitive applications such as artificial intelligence.

```air
_ do _[
    _set_steps set _ import .resource.set_steps,
    _ set_steps 100,
    true loop []
]
```

## Roadmap

1. **Enhance Language Expressiveness**  
   Focus on expanding core expressiveness to lay the foundation for subsequent capabilities.

2. **Introduce Abstract Semantics and Program Optimization Framework**  
   Introduce an abstract semantics model based on "concrete value + abstract constraint", enabling optimization of values within the same class while maintaining semantic equivalence, and build a general program optimization framework based on this.

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
