# Air Programming Language

## Design Goals

- **Universal**  
  The boundaries of a programming language are the boundaries of a programmer's ability, so the language should be applicable to any need and should not self-limit.

- **Reliable**  
  Continuous error accumulation will eventually make a system unusable. Only reliable systems can develop sustainably, so the language should be able to prevent and manage errors.

- **Lean**  
  The common language among programmers should be easy to learn, understand, and use, so the language should avoid unnecessary complexity.

## Language Features

### Syntax

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
➔ >=

a.b.c
➔ a.b.c

'[0, 1, 2]'
➔ [0, 1, 2]

'"a"'_(this is a comment)_[X3f ' " ) ( sp]_"'a'"
➔ "a"?'")( 'a'

'abcdefghijklmnopqrstuvwxyz'_
|"()[]{}<>\|/'"_
|'"`^*+=-~_.,:;!?@#$%&'_
|(this is a comment)_
|[sp 0 1 2 3 4 5 6 7 8 9]_
|'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
➔ abcdefghijklmnopqrstuvwxyz()[]{}<>\|/'"`^*+=-~_.,:;!?@#$%& 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ
```

**text**

`"text"`

```air
"🜁: Alchemical Symbol For Air"
➔ 🜁: Alchemical Symbol For Air

"'a'"_(this is a comment)_[X1f701 ' " sp ht cr lf]_'"a"'
➔ 'a'🜁'" \t\r\n"a"

    "()[]{}<>\|/'"_
    |'"`^*+=-~_.,:;!?@#$%&'_
    |(this is a comment)_
    |[X1f701 ' " sp ht cr lf]
➔ ()[]{}<>\|/'"`^*+=-~_.,:;!?@#$%&🜁'" \t\r\n
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

**solve**

- `? function output`
- `output function ?`

```air
? * 21
true is_carmichael_number ?
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

### Semantics

**key**

1. `_a` ➔ `a`
2. `.a` ➔ `.a`
3. `a` ➔ `v`, where `v` is the value bound to key `a` in the context

**quote**

`_(v)` ➔ `v`

**call**

`_ f i` ➔ `f'(i')`, where `x'` denotes the result of evaluating `x` (the same applies below)

**solve**

`? f o` ➔ `i`, where `f'(i) = o'` is a fact in the configuration's fact database

**cell**,

`.(v)` ➔ `.(v')`

**pair**

`v1 : v2` ➔ `v1' : v2'`

**list**

`[v1, v2, ..., vn]` ➔ `[v1', v2', ..., vn']`

**map**

`{k1 : v1, k2 : v2, ..., kn : vn}` ➔ `{k1 : v1', k2 : v2', kn : vn'}`

**others**

`v` ➔ `v`

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

## Roadmap

Many goals do not yet have clear design proposals. The following directions will be explored in the future:

- Logical framework, paraconsistent logic, and error management
- Non-computable semantics based on solving
- Program analysis and optimization based on abstract interpretation theory
- Algorithm complexity analysis and resource management
- Concurrency model

## Installation and Running

Install:

```bash
cargo install airlang_bin
```

Run the interactive interpreter:

```bash
airlang_bin
```

Run a file:

```bash
airlang_bin path/to/your/file.air
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
