# Air 编程语言

## 设计目标

- **普适**  
  编程语言的边界就是程序员能力的边界，所以语言应该适用于任何需求，不该自我设限。
- **可靠**  
  持续性的错误积累最终会导致系统不可用，只有可靠的系统才能可持续发展，所以语言应该能规避和管控错误。
- **精简**  
  程序员间的共同语言应该容易学习理解和使用，所以语言应该避免非必要的复杂性。

## 语言特性

### 语法

**单元**

`.`

**比特**

- `true`
- `false`

**键**

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

**文本**

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

**整数**

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

**小数**

- `decimal'0.'`
- `0-1.` = `decimal'-1.'`

```air
12.3
0-12.3
decimal'-12.3'
0-E-12*3.456
decimal'-E-12*3.456'
```

**字节**

`byte'00'`

```air
byte'B00001111'
byte'X00ffff'
```

**单元格**

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

**配对**

`left : right`

```air
a : 1
a : b : c
```

**列表**

- `[v1, v2, ..., vn]`
- `#[v1 v2 ... vn]` = `[v1, v2, ..., vn]`

```air
[0, 1, 2]
[., false, 0, '',]
#[git commit --amend --no-edit]
```

**映射**

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

**引用**

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

**调用**

- `_ function input`
- `input function _`
- `left function right` = `_ function left : right`

```air
_ not true
1 + 1
a and b or c
```

**求解**

- `? function output`
- `output function ?`

```air
? * 21
true is_carmichael_number ?
```

**注释**

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

### 语义

**键**

1. `_a` ➔ `a`
2. `.a` ➔ `.a`
3. `a` ➔ `v`，`v` 为上下文中键 `a` 所绑定的值

**引用**

`_(v)` ➔ `v`

**调用**

`_ f i` ➔ `f'(i')`，其中 `x'` 表示 `x` 求值的结果，下同

**求解**

`? f o` ➔ `i`，其中 `f'(i) = o'` 是配置事实库中的一条事实

**单元格**

`.(v)` ➔ `.(v')`

**配对**

`v1 : v2` ➔ `v1' : v2'`

**列表**

`[v1, v2, ..., vn]` ➔ `[v1', v2', ..., vn']`

**映射**

`{k1 : v1, k2 : v2, ..., kn : vn}` ➔ `{k1 : v1', k2 : v2', kn : vn'}`

**其他**

`v` ➔ `v`

### 上下文

上下文是执行过程中的局部信息坏境，在核心语义中可以通过键访问上下文，函数也支持感知或更新上下文。可通过 `get` 函数读取上下文中的变量，或通过 `set` 函数更新上下文中的变量，亦可通过 `which` 函数指定上下文。我们基于函数的这项特性实现了各种控制流函数，包含顺序执行 `do`，条件执行 `test`，模式匹配 `match`，循环 `loop`，迭代 `iterate` 等，并在初始上下文中提供了最常用和最必要的核心函数。

```air
_ do _[
    _sum set 0,
    100 iterate _i : _[
        _sum set sum + i
    ],
    sum
]
```

### 配置

配置是执行过程中的全局信息坏境，通过仅追加与局域覆盖等机制，兼顾了灵活性和可预测性。可通过 `import` 函数导入配置项，或通过 `export` 函数导出配置项，亦可通过 `with` 函数局域覆盖配置项。我们将基于配置机制实现模块管理、测试框架和错误处理等特性，并在初始配置中提供了原生函数和标准库。

```air
_ do _[
    _push set _ import .list.push,
    .list.add export push,
    .list.append export push,
]
```

## 路线图

许多目标还没有确定清晰的设计方案，未来将探索以下方向

- 逻辑框架，次协调逻辑和错误管理
- 基于求解的不可计算语义
- 基于抽象解释理论的程序分析和优化
- 算法复杂度分析和资源管理
- 并发模型

## 安装运行

安装：

```bash
cargo install airlang_bin
```

运行交互式解释器：

```bash
airlang_bin
```

运行文件：

```bash
airlang_bin path/to/your/file.air
```

## 许可证

您可以选择使用

* Apache 2.0 许可证
  ([LICENSE-APACHE](LICENSE-APACHE) 或 <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT 许可证
  ([LICENSE-MIT](LICENSE-MIT) 或 <http://opensource.org/licenses/MIT>)

## 贡献

除非您明确说明，否则您有意提交以纳入作品的任何贡献（如 Apache-2.0 许可证中所定义），均应按照上述方式获得双重许可，无任何附加条款或条件。
