# struct

## 1. 概述

struct，结构体

- 自定义的数据类型
- 为相关联的值命名，打包组成有意义的组合

## 2. struct的使用

### 2.1. 定义struct

- 使用`struct`关键字，并为整个struct命名
- 在花括号里，为所有字段（field）定义名称和类型

例如：

```rust
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

在定义每一个字段的后面都需要包含一个“逗号”，即使是最后一个字段也需要包含逗号

### 2.2. 实例化struct

想要使用struct，需要创建struct的实例：

- 为每个字段指定具体值
- 无需按照声明的顺序进行指定

### 2.3. 取得struct里面的某个值

使用点标记法，如下代码

```rust
let mut user1 = User{
    email: String::from("some@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

user1.email = String::from("another@example.com")
```

> 注意：一旦struct的实例是可变的，那么实例中所有字段都是可变的

### 2.4. struct作为函数的返回值

struct可以作为函数的返回值，如下代码

```rust
fn build_user(email: String, username: String)-> User{
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

当字段名与字段值对应的变量名相同时，就可以使用字段初始化简写的方式，如下代码

```rust
fn build_user(email: String, username: String)->User{
User{
    email,
    username,
    active: true,
    sign_in_count: 1,
}
}
```

### 2.5. struct更新语法

当你想基于某个struct实例来创建一个实例的时候，可以使用struct的更新语法，我们先看不使用更新语法的代码

```rust
let user2 = User{
    email: String::from("another@example.com"),
    username: String::from("anotheruser567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count
};
```

使用更新语法之后，可以写成如下代码

```shell
let user2 = User{
    email: String::from("another@example"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### 2.6. Tuple struct

可以定义Tuple的struct，叫做tuple struct

- Tuple struct整体有个名，但里面的元素没有名
- 适用：想给整个tuple起名，并让它不同于其他tuple，而且又不需要给每个元素起名

定义tuple struct：使用struct关键字，后面是名字，以及里面元素的类型

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

black和origin是不通的类型，是不通的tuple struct实例

### 2.7. Unit-Like Struct（没有任何字段）

可以定义没有任何字段的struct，叫做Unit-Like struct（因为与()，单元类型类似）

- 适用于某个类型上实现某个trait，但是在里面又没有想要存储的数据

### 2.8. struct数据的所有权

我们先看如下代码

```rust
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

上面代码中，字段使用了String而不是&str：

- 该struct实例拥有其所有的数据
- 只要struct实例是有效的，那么里面的字段数据也是有效的

struct里也可以存放引用，但需要使用生命周期（生命周期后面会讲到）

- 生命周期保证只要struct实例是有效的，那么里面的引用也是有效的

- 如果struct里存储引用，而不使用生命周期，那么就会报错
