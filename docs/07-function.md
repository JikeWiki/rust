# 函数

## 1. 函数的声明

声明函数使用`fn`关键字，依照惯例，针对函数名和变量名，Rust使用snake case 命名规范，即所有字母都是小写，单词之间使用下花香分开。如下

```rust
fn main() {
    println!("Hello, world!");
    annother_function();
}

fn annother_function() {
    println!("Another function.");
}
```


## 2. 函数的参数

parameters是我们在定义函数时的参数，叫形参；arguments是我们在调用函数时的实际参数，叫实参。在函数的签名里，必须声明每个参数的类型，如下

```rust
fn main() {
    println!("Hello, world!");
    annother_function(5, 6); // arugment is 5 and 6
}

fn annother_function(x: i32, y: i32) {
    // parameter is x and y
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}
```

## 3. 函数中的语句（statement）与表达式（expression）

函数有一系列的语句组成，可选的可以由一个表达式结束。语句是执行一些动作的指令，函数的定义也是语句，语句不返回值，所以不可以使用let将一个语句赋给一个变量。rust是一个基于表达式的语言，表达式会执行计算并产生一个值。


示例：

```rust
fn main() {
    let x = 5;
}
```

以上代码中，main函数的整体也是语句。其中`let x = 5;`是一条语句 ，该部分的右边是一个表达式，该表达式的值就是`5`，如果我们把该行改为`let x = 5 + 6;`，那么表达式就是`5 + 6`。表达式可与作为语句的一部分，调用函数是表达式，调用`println!`宏也是表达式。



## 3. 函数的返回值

在`->`符号后面声明函数返回值的类型，但是不可以为返回值命名。在rust里，通常返回值就是函数体里面最后一个表达式的值。如果想提前返回，需使用`returen`关键字，并指定一个值，大多数函数默认使用最后一个表达式作为返回值。如下例子：

```rust
fn plus_five(x: i32) -> i32 {
    x + 5
}

fn main() {
    let x = plus_five(6);
    println!("The value of x is: {}", x);
}
```

在上面代码中，`five`函数的最后一个表达式值是`x = 5`，所以`x + 5`是该函数的返回结果。注意的是`x + 5`后不能加`;`，否则返回的是`tuple`，而不是`i32`。


## 4. 函数的注释


函数可以使用单行注释或者多行注释的方式，单行注释符号为`//`，多行注释以`/*`开头，以`*/`结尾，如下示例

```rust
// 这是单行注释
fn plus_five(x: i32) -> i32 {
    x + 5
}

/**
 * 这是多行注释
 */
fn main() {
    let x = plus_five(6);
    println!("The value of x is: {}", x);
}
```















