# Catly

[![Build & Test](https://github.com/Thaumy/catly/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/Thaumy/catly/actions/workflows/build_and_test.yml)

Catly 是一门图灵完备、惰性求值、静态强类型的函数式语言。

## 语法概览

### Comment

注释

在 Catly 中，注释是 `#` 至换行符 `\n` 之间的所有内容。

例外：如果 `#` 后不存在 `\n`，那么 `#` 后面的所有内容都将被视为注释。\
注意：`#` 后的空格 `` 是必须的。

```
# This is a comment
```

### Naming conventions

命名约定

`[0-9a-zA-Z]` 是 Catly 中名的合法字符。

使用 camelCase 对一个**值**命名，其首字符必须为**小写字母**：

例如：

- a
- abc
- a1b2c3
- helloWorld

使用 CamelCase 对一个**类型**命名，其首字符必须为**大写字母**：

例如：

- A
- Abc
- A1b2c3
- HelloWorld

### Primitive data types

基本数据类型

Int, Unit 和 Discard 是 Catly
中的基本数据类型，它们是组成所有语言结构的基础元素。

```Catly
# Int
1
```

```Catly
# Unit
()
```

```Catly
# Discard
_
```

`Discard` 常用于_模式匹配_，它是一个通配值，和任何值相等，但不允许被求值。

### Let expression

Let 表达式

`let` `in` 关键字用于构造 Let
表达式，它用于将**名**和**值**进行绑定，并允许在随后的表达式中通过引用该名取得其所绑定的值。

```Catly
# The value of this expression is 1
let a = 1 in a
```

同样支持嵌套的 Let 绑定：

```Catly
let a = 1 in let b = 2 in ()
```

但 Catly 提供了语法糖，它与上一种写法等效：

```Catly
let a = 1, b = 2 in ()
```

重复对相同的名进行绑定只会在新的作用域环境中建立新的绑定关系，而不影响原来的绑定。

在 Catly 中，值是**不可变**的。

Catly 允许 Let 表达式的最后一个绑定以 `,` 结尾，但 `,` 后必须跟随 `in`
关键字，例如：

```Catly
let a = 1, b = 2,in ()
```

这种写法能够为代码生成带来便利。

### If expression

If 表达式

关键字 `if` `then` `else` 用于构造 If 表达式。

`eq` 是 Catly
中的内置函数，用于判断两个值的相等性。当两个值在类型和值上均相等时，`eq`
返回真。否则，`eq` 返回假。

```Catly
# The value of this expression is 4
if eq 1 2 then 3 else 4
```

有关布尔类型的讨论参见 Type definition 部分。

### Lambda expression

λ 表达式

函数在 Catly 中是 First class 的，所有的函数均由 λ 表达式构造。

λ 表达式形如 `name -> expression`，其中 name 是参数，而 experssion
是使用该参数的表达式。

`add` 是 Catly 中的内置函数，它对两个参数（Int 类型）求和并将结果返回。

```Catly
a -> b -> add a b
```

λ 表达式是 Currying 的，这意味着上述写法与如下形式等效：

```Catly
a -> (b -> (add a) b)
```

将值应用于 λ 表达式：

```Catly
# The value of this expression is 3
(a -> b -> add a b) 1 2
```

λ 表达式的参数支持弃元，被弃元的参数不能在表达式体中被引用：

```Catly
# The value of this expression is 1
(_ -> 1) 0
```

### Struture

结构

结构是一组名和值的集合。

```Catly
{ a = 1 }
```

```Catly
{ a = 1, b = 2, c = 3 }
```

与 Let 表达式相似，Catly 也允许结构的最后一个字段以 `,` 结尾，但 `,` 后必须跟随
`}`，例如：

```Catly
{ a = 1, b = 2, c = 3,}
```

### Pattern matching

模式匹配

关键字 `match` `with`
用于构造模式匹配，模式匹配顺序地匹配模式和解构类型，首个符合匹配的模式的表达式将被求值。

模式匹配在 Catly 中是表达式。

使用模式匹配来匹配值：

```Catly
match x with
| 1 -> add x 1 
| 2 -> 1 
| _ -> 0
```

下表显示了表达式在 x 的不同取值下的值：

|  x   | value |
| :--: | :---: |
|  1   |   2   |
|  2   |   1   |
| 其他 |   0   |

模式匹配可用作解构值：

```Catly
match x with
| { a = 1, b = 1 } -> a 
| { a = 2, b = 2 } -> b 
| { a = _, b = 3 } -> 0 
| y -> add y 1
```

注意：模式匹配仅用于匹配**常量**。\
上述代码中的模式 `y` 并非是对某个值的引用，当没有模式与前三种情况匹配时，`y`
模式将被匹配，`y` 将在随附的表达式 `add y 1` 中被绑定为 `x` 的值。

当没有模式符合匹配时(即 Non-exhaustive)，求值将发生异常。

### Top-level expression definition

顶层表达式定义

关键字 `def`
用于在顶层对表达式和名进行绑定，这种绑定的作用域是跟随其的整个代码序列。

例如：

```Catly
def a = 1
def f = x -> y -> add x y
```

这些名可以在绑定完成的那一刻起在后续环境中可用。

main 函数是 Catly 的入口点，它也是一种顶层定义：

```Catly
def main = x -> 1
```

顶层定义不能被嵌套在任何已有的定义之中，这意味着这些定义只能出现在整个抽象语法树的最上层。如下的定义是非法的：

```Catly
# This code is illegal!!!
def i =
  def j = 1
  j
```

### Type annotation

类型标注

Catly 是强类型的，当 Catly 无法推断某个值的类型时，需要使用类型标注。

例如，函数 `f` 适用于任何参数类型，Catly 会在类型检查期间对其进行推导：

```Catly
def f = x -> ()
```

使用类型标注限定 `f` 的参数类型，使其只能作用于 `Int` 类型的参数：

```Catly
def f = (x: Int) -> ()
```

在模式匹配中类型标注可用作类型匹配，例如：

```Catly
match b with
| (_: True) -> 123
| (_: False) -> 456
```

类型标注适用于所有表达式。

### Type definition

类型定义

关键字 `type` 用于定义新类型(new type)。同顶层表达式定义一样，类型定义也是顶层定义。

创建一个新的 A 类型，它是基本类型 Unit 的包装：

```
type A = Unit
```

构造 A 类型值的方式与构造 Unit 值的方式相同，但 A 与 Unit
并不通用。为了正确构造类型为 A 的值，需要使用类型标注：

```Catly
def newA = (): A
```

同样可以基于结构类型定义新的类型：

```Catly
type IntPair = { l: Int, r: Int }
def newIntPair = l -> r -> { l = l, r = r }: IntPair
```

结构类型在 Catly 中被视作**积类型**，Catly
还支持**和类型**，和类型是几种类型之一的类型，例如：

```Catly
type IntOrUnit = Int | Unit
def newIntOrUnit = i -> (if eq i 0 then 1 else ()): IntOrUnit
```

对和类型的结构可使用模式匹配：

```Catly
match x: IntOrUnit with
| i: Int -> 1
| u: Unit -> 0
```

积类型和和类型共同组成了 Catly 的代数数据类型系统。

Catly 内置了用于 If 表达式的布尔类型：

```Catly
type True = Int
type False = Int
type Bool = True | False

def true = 1: True
def false = 0: False
```

有关更多内置类型的内容，请参考 Catly 标准库。

### Catly 类型系统中的高级内容

Catly具备运行时类型系统。当对基本数据类型进行类型标注时，类型将附加于值。

附加有类型的值可被 match 表达式用作类型解构，例如：

```Catly
type IntOrUnit = Int | Unit
let x = 5: Int

# The value of this expression is 1
match x with
| _: Int -> 1
| _: Unit -> 2
```

重复的类型标注只会使得第一个类型标注被附加在值上，例如：

```Catly
(x: Int): IntOrUnit # It's equivalent to `x: Int`
```

所以，当和类型的组成类型在值结构相同的情况下，应显式使用类型标注以确保值的类型能在运行时被识别，例如：

```Catly
type A = Unit
type B = Unit
type AB = A | B
def a = (): A
def b = (): B

# The value of this expression is 1
match a with
| _: A -> 1
| _: B -> 2

# The value of this expression is 2
match b with
| _: A -> 1
| _: B -> 2

# The value of this expression is 3
match (): AB with
| _: A -> 1
| _: B -> 2
| _ -> 3
```

### Built-in function

内置函数

Catly
的内置函数用于提供不可分割的基本算术操作。当集齐所有操作数时，内置函数将立即求值。

| 名称 |  功能  |         类型         |
| :--: | :----: | :------------------: |
| neg  |  取负  |      Int -> Int      |
| add  |  求和  |  Int -> Int -> Int   |
| sub  |  求差  |  Int -> Int -> Int   |
| mul  |  求积  |  Int -> Int -> Int   |
| div  |  整除  |  Int -> Int -> Int   |
| mod  |  取模  |  Int -> Int -> Int   |
| rem  |  取余  |  Int -> Int -> Int   |
|  gt  |  大于  |  Int -> Int -> Bool  |
|  eq  |  等于  |  Int -> Int -> Bool  |
|  lt  |  小于  |  Int -> Int -> Bool  |
| not  | 逻辑非 |     Bool -> Bool     |
| and  | 逻辑与 | Bool -> Bool -> Bool |
|  or  | 逻辑或 | Bool -> Bool -> Bool |

### Standard library

标准库

Catly 标准库将在代码执行前嵌入程序结构，用于提供一些预置的算术基础设施。

<details>
<summary>Expand</summary>

```Catly
def neg: Int -> Int = _ # compiler built-in
def add: Int -> Int -> Int = _
def sub: Int -> Int -> Int = _
def mul: Int -> Int -> Int = _
def div: Int -> Int -> Int = _
def mod: Int -> Int -> Int = _
def rem: Int -> Int -> Int = _

def gt: Int -> Int -> Bool = _
def eq: Int -> Int -> Bool = _
def lt: Int -> Int -> Bool = _

def not: Bool -> Bool = _
def and: Bool -> Bool -> Bool = _
def or: Bool -> Bool -> Bool = _

type True = Int
type False = Int
type Bool = True | False

def true = 1: True
def false = 0: False

type EmptyList = Unit
type IntCons = { head: Int, tail: IntList }
type IntList = IntCons | EmptyList

def emptyList = (): EmptyList
def intCons = h -> t -> { head = h, tail = t } : IntCons

type Fraction = { n: Int, d: Int }

def gcd = a -> b ->
    if eq b 0 then
        a
    else
        gcd b (rem a b)

def fraction = n -> d ->
    if gt n 1000 then
        let
            g = gcd n d
        in
            { n = div n g, d = div d g }: Fraction
    else
        { n = n, d = d }: Fraction

def int2F = i ->
    fraction i 1
```

</details>
