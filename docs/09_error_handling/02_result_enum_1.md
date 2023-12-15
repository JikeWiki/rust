# Result枚举与可恢复错误1

## 一、概述

通常情况下，错误都没有严重到要停止整个应用的地步。当某个函数运行失败，一般都有一些可以简单解释并做出可以做响应的原因所引起的，比如说我们程序想打开某个文件，但这个文件却不存在，这个时候通常是要创建该文件，而不是终止程序。在Rust里我们可以使用`Result`枚举类型，来处理可能失败的情况。该枚举类型的结构如下

```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T`：操作成功情况下，Ok变体里返回的数据类型
`E`：操作失败的情况下，Err变体里返回的错误的类型

## 二、错误的处理

### 2.1 通常的处理方式

处理Result的一种方式：match表达式。和`Option`枚举一样，Result及其变体也是由`prelude`代码作用域。错误处理过程如下示例代码

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error)
        }
    };
}
```

### 2.2 匹配不同的错误

可以在`match`里嵌套`match`，如下示例代码

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 文件不存在创建文件
            ErrorKind::NotFound => macth File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            // 其他的错误类型，可以自定义变量名，这里叫做 other_error
            other_error => panic!("Error opening the file: {:?}", other_error),
        }
    };
}
```

我们可以使用`unwrap_or_else`方式实现以上代码实现的功能，如下示例代码

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening the file: {:?}", error);
        }
    });
}
```

### 2.3 unwrap和expect

`unwrap`可以被看作`match`表达式的一个快捷方法。如果`Result`的结果是`OK`，则返回OK的值，如果`Result`的结果是`Err`，则调用`panic!`宏。如下示例代码

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

我们还可以使用`expect`方法，改方法和`unwrap`类似，但可指定错误信息，如下示例代码

```Rust
let f = File::open("hello.txt").expect("无法打开文件");
```
