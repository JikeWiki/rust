# vector

## 一、概述

Vector<T>，叫做Vector，又一个标准库提供，可存储多个值，只能存储相同的数据类型，这些值在内存中连续存放。

使用`Vec::new`创建Vector，如下示例代码

```Rust
fn main() {
    let v: Vec<i32> = Vec::new();
}
```

也可以使用初始值创建`Vec<T>`，使用`vec!`宏，如下示例代码

```Rust
fn main() {
    let v = vec![1, 2, 3];
}
```

## 二、更新vector

可以使用`push`方法添加元素，如下示例代码

```Rust
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
}
```

## 三、读取vector的元素

两种方式可以引用vector里的值，索引、get方法，如下示例代码

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element id {}", third);

        None => println!("There is no third element");
    }
}
```

遍历vector

```Rust
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
```

我们还可以在编译的过程中修改vector元素的值

```Rust
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in v {
        println!("{}", i);
    }
}
```

## 四、删除vector

与其他元素struct一样，当vector离开作用域后，它就被清理掉，它所有的元素也被清理掉。

## 五、所有权和借用规则

不能在同一作用域内同时拥有可变和不可变引用。如下示例代码

```rust
fn main() {
    // 声明了一个可变的vec
    let mut v = vec![1, 2, 3, 4, 5];
    // 引用了一个不可变的借用
    let first = &v[0];
    // 又向v中添加值，这里发生了可变的借用，将导致报错
    v.push(6);
    println!("The first element is {}", first);
}
```
