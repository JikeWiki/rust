# 变量与可变性


## 1. 变量

- 声明变量使用let

- 默认情况下，变量是不可变的


- 在声明变量的时候，在变量的前面加上`mut`，就可以使变量可变。


我们先来回顾一下以下的代码

```rust
fn main() {
    let x = 5;
    println!("The value of x is {}!", x);
}
```

我们先声明了一个`x`变量，并打印到终端上，`x`默认被推倒为`i32`类型，这个变量默认就是不可变的。因为没有`mut`关键字，如下示例代码

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is {}!", x);
    x = 6;
    println!("The new value of x is {}!", x);
}
```

## 2. 常量

常量（constant），常量在绑定值以后也是不可变的，但是他与变量有很多区别

- 常量不可以使用mut，常量永远都是不可变的

- 声明常量使用constant关键字，它的类型必须被标注清楚

- 常量可以在任何作用域内进行申明，包括全局作用域

- 常量只可以绑定到常量表达式，无法绑定到函数调用结果或只能在运行时才能计算出的值


在程序运行期间，常量在其作用域内一直有效，Rust里常量的命名规范是使用全大写字母，每个单词之间使用下划线分开，如下例子

```rust
const MAX_POINTS: u32 = 10_0000;
```

在rust里数字可以添加下滑线，来增强可读性，如以上的值时十万。


## 3. shadowing

可以使用相同的名称声明新的变量，新的变量就会shadow（隐藏）之前的同名变量，看到以下代码，如果我们执行编译将会报错，因为变量默认不可变

```rust
fn main() {
    let x = 5;
    x = 6;
    println!("The value of x is {}!", x);
}
```

但如果我们使用重新声明的方式，代码将能编译通过

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is {}!", x);
}
```

如果在`let x = 6;`这行代码之后，再使用到`x`，该变量就是就是最新的变量了。因为后面`x`的声明把前面的`x`进行shaw（隐藏）了。show（隐藏）和把变量标记为mut是不一样。

- 如果不使用let关键字，那么重新给飞mut的变量赋值会导致编译错误
- 而使用let声明的同名新变量，也是不变的
- 使用let声明的同名新变量，它的类型可以和之前不同


示例代码：

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The spaces value of x is {}!", spaces);
}
```


在以上代码中，第一个`spaces`是字符串类型，而第二个`spaces`则变成了`usize`类型（区分计算机位数的无符号整型）。如果我们把代码改成如下内容

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("The spaces value of x is {}!", spaces);
}
```

此时编译会报错，因为在`spaces = spaces.len();`这行代码中，左边是包含`mut`关键字的字符串类型，右边的执行结果是`usize`类型。


> rust的show功能，让我们可以灵活地复用变量名。





