# Catly

Catly 是一门图灵完备、惰性求值、静态强类型的函数式语言。

## 语法概览

### Comment

注释

在 Catly 中，注释是 `# ` 至换行符 `\n` 之间的所有内容。  

例外：如果 `# ` 后不存在 `\n`，那么 `# ` 后面的所有内容都将被视为注释。  
注意：`#` 后的空格 ` ` 是必须的。

```
# This is a comment
```

### Naming conventions

命名约定

`[0-9a-zA-Z]` 是 Catly 中名的合法字符。

使用 camelCase 对一个 值 命名，其首字符必须为**小写字母**：

例如：

* a
* abc
* a1b2c3
* helloWorld

使用 CamelCase 对一个 类型 命名，其首字符必须为**大写字母**：

例如：

* A
* Abc
* A1b2c3
* HelloWorld

### Primitive data types

基本数据类型

Int, Unit 和 Discard 是 Catly 中的基本数据类型，它们是组成所有语言结构的基础元素。

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

`Discard` 常用于*模式匹配*，它是一个通配值，和任何值相等，但不允许被求值。

### Let expression

Let 表达式

`let` `in` 关键字用于构造 Let 表达式，它用于将**名**和**值**进行绑定，并允许在随后的表达式中通过引用该名取得其所绑定的值。

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

Catly 允许 Let 表达式的最后一个赋值以 `,` 结尾，但 `,` 后必须跟随 `in` 关键字，例如：

```Catly
let a = 1, b = 2,in ()
```

这种写法能够为代码生成带来便利。

### If expression

If 表达式

`if` `then` `else` 关键字用于构造 If 表达式。

`eq` 是 Catly 中的内置函数，用于判断两个值的相等性。当两个值在类型和值上均相等时，`eq` 返回真。否则，`eq` 返回假。

```Catly
# The value of this expression is 4
if eq 1 2 then 3 else 4
```

有关布尔类型的讨论参见 Alegbraic data type 部分。

### Lambda expression

λ 表达式

函数在 Catly 中是 First class 的，所有的函数均由 λ 表达式构造。

λ 表达式形如 `name -> expression`，其中 name 是参数，而 experssion 是使用该参数的表达式。

`add` 是 Catly 中的内置函数，它对两个参数（Int 类型）求和并将结果返回。

```Catly
a -> b -> add a b
```

λ 表达式是 Currying 的，这意味着上述写法只是如下形式的语法糖：

```Catly
a -> (b -> (add a) b)
```

将值应用于 λ 表达式：

```Catly
# The value of this expression is 3
(a -> b -> add a b) 1 2
```

### Struture

结构体

结构体是一组名和值的集合。

```Catly
{ a = 1 }
```

```Catly
{ a = 1, b = 2, c = 3 }
```

与 Let 表达式相似，Catly 也允许结构的最后一个字段以 `,` 结尾，但 `,` 后必须跟随 `}`，例如：

```Catly
{ a = 1, b = 2, c = 3,}
```

### Pattern matching

模式匹配

`match` `with` 关键字用于构造模式匹配，用于顺序地匹配模式和解构类型。  

模式匹配在 Catly 中是表达式。

使用模式匹配来匹配值：

```
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

模式匹配还被用作解构类型：

```
match x with
| { a = 1, b = 1 } -> a 
| { a = 2, b = 2 } -> b 
| { a = _, b = 3 } -> 0 
| y -> add y 1
```

注意：模式匹配仅用于匹配**常量**。  
上述代码中的模式 `y` 并非是对某个值的引用，当没有模式与前三种情况匹配时，`y` 模式将被匹配，`y` 将在随附的表达式 `add y 1` 中被绑定为 `x` 的值。

当没有模式符合匹配时，求值将发生异常。

