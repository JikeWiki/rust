# 泛型数据类型

## 一、概述

泛型是具体类型或其他属性的抽象代替。能提高代码复用的能力，用于处理重复代码的问题。我们可以理解为，我们编写的代码不是最终代码，而是一种模板，里面有一些“占位符”。编译器在编译的时会讲“占位符”替换为具体的类型。例如

```Rust
fn lagest<T>(list: &[T]) -> T {}
```

泛型的类型参数通常很短，通常是一个字母。同时使用`CameCase`命名规范，而字母`T`是`type`的缩写，所以通常使用`T`作为泛型参数。

## 泛型的使用

### 2.1 函数中定义的泛型

函数中使用泛型，通常是参数类型手机用泛型或者返回类型使用泛型，通常在函数名的后面在`<>`里声明泛型，如下示例

```ruby
fn largest<T>(list: &[T]) -> {
    ...
}
```

### 2.2 在结构体中使用泛型

在结构体名称后面使用`<>`申明泛型类型，泛型类型可以用于一个或者多个字段，如下示例代码

```Rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

我们还可以声明多个泛型数据，如下示例代码

```Rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mix = Point { x: 5, y: 10.0 };
}
```

可以使用多个泛型类型参数，但是太多类型参数，可能回影响代码的阅读。

### 2.3 Enum定义中的泛型

可以让枚举的变体持有泛型数据类型，如下示例

```Rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {}
```

### 2.4 在方法定义中的泛型

为struct或enum实现方法的时候，可在定义中使用泛型，如下示例代码

```Rust
struct Point<T> {
    x: T,
    y: T,
}

// 使用泛型实现struct的方法，在所有的Point<T>类型中都包含x方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 使用具体类型实现struct的方法，只有在Point<i32>中才有x1方法，其他的Point<T>类型中不包含x1方法
impl Point<i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}
```

把`T`放在impl关键字后面，表示在类型`T`上实现方法。如下示例代码

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: "Hello", y: "c" };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## 三、泛型代码的性能

使用泛型的代码和使用具体类型的代码运行速度是一样的。因为rust在编译的过程中会执行一个单态化（monomorphization）的过程，即在编译时将泛型替换为具体类型的过程。下面是一段使用泛型原始代码

```rust
fn mian() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

在编译过程中，将泛型进行单态化，变更为类似如下代码

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn mian() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
