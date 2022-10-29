# 用到模式匹配的地方

## 一、概述

模式匹配是rust的一种特殊语法，用于匹配复杂和简单类型的结构，将模式与匹配表达式和其他结构结合使用，可以更好地控制程序的控制流。模式由以下元素（的一些组合）组成

- 字面值
- 解析的数组、enum、struct和tuple
- 变量
- 通配符
- 占位符

想要使用模式，需要将其与某个值进行比较，如果模式匹配，就可以在代码中使用这个值的相应部分。下文将介绍rust中使用到模式匹配的地方。

## 二、match的Arm（分支）

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

match表达式的要求是详尽，即包含所有的可能性。一个特殊的模式是`_`（下划线），它会匹配到任何东西，不会绑定到变量，所以通常用户match的最后一个分支，或者用于忽略某些值。

## 三、条件if let表达式

`if let`主要是作为一种简短的方式来等价的代替只有一个匹配项的match。`if let`可选可以拥有`else`，包括`else if`和`else if let`，但是`if let`不会检查穷尽性。

## 四、while let条件循环

只要模式匹配继续满足匹配的条件，那它允许while循环一直运行。如下示例

```rust
fn main() {
    let mut stack = Vec::new();

    statck.push(1);
    statck.push(2);
    statck.push(3);

    while let Some(top) = stack.pop() {
        println("{}", top);
    }
}
```

## 五、for循环

for循环是rust中最常见的循环，for循环中，模式就是紧随for关键字后的值。如下示例代码

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println("{} is at index {}", value, index);
    }
}
```

## 六、let语句

let语句也是模式，官方写法为`let PATTERN = EXPRESSION;`，如下示例代码

```rust
fn main() {
    let a = 5;
    
    let (x, y, z) = (1, 2, 3);
}
```

## 七、函数参数

函数参数也是模式，如下示例代码

```rust
fn foo(x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```
