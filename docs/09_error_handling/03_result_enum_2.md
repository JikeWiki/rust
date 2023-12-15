# Result枚举与可恢复错误2

## 一、传播错误

我们除了在函数出处处理错误；还可以将错误返回给函数的调用者，让调用者进一步进一步处理错误，这个过程就叫做传播错误。如下示例代码

```Rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn main() {
    let result = read_username_from_file();
}
```

## 二、?运算符

### 2.1 ?运算符的使用

?运算符是传播错误的一种快捷方式。如果`Result`的结果是`OK`，`OK`中的值就是表达式的结果，然后继续执行程序；如果`Result`是`Err`，`Err`就是整个函数的返回值，就像使用了`return`。

那么，在“一”中的函数可以简写如下

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

### 2.2 ?与from

`Trait std::convert::From`上的`from`函数，用于错误之间的转换。

被`?`所应用的错误，会隐式的被`from`函数处理，当`?`调用`from`函数时，它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型。

这种情况用于：针对不同错误类型，返回同一种错误类型。只要相关的错误类型实现了转换为所返回的错误类型的`from`函数，就可以被转换为返回的错误类型。如同“二”中的示例。

我们可以把“二”中的代码改为链式调用，如下内容

```Rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### 2.3 ?运算符只能用于返回Result的函数

如果我们直接在`main`使用`?`运算符，如下代码，将会报错

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

此时的`main`函数返回单元类型`()`，相当于什么都没返回，因此会报错。我们可以修改函数如下

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("hello.txt")?;
    Ok(())
}
```

此时`main`函数返回一个`Result`枚举，编辑将不会报错。枚举的变体为单元类型`()`或者`Box<dyn Error>`。`Box<dyn Error>`是一个trait对象，此处可以简单理解为：“任意可能的错误类型”。
