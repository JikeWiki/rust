# 定义枚举

IP地址一共有两种类型：IPv4、IPv6，所以IP地址要么是`IPv4`要么是`IPv6`，就因为这个特性，IP地址就非常适合用枚举来表示，我们把枚举所有可能的值叫做枚举的变体。我们可以使用如下代码定义IP地址的枚举数据

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

可以使用以下的方式使用枚举值

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

枚举的变体都位于枚举所在标准命名空间的下面。以下是枚举值的应用示例代码

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

我们还可以使用struct来存储枚举类型，如下代码

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}
```

不过在rust里，允许直接将数据附加刀片枚举的变体中，如

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

优点：

- 不需要额外的struct存储相关的数据
- 每个变体可以用有不同类型以及关联的数据的量

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"))
}
```

我们再来看标准库中的IPAddr

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --sn
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

不论是字符串、数值、结构体，甚至是另外一种枚举类型，都可以作为枚举的变体。如下代码

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
}
```

还可以使用`impl`关键字定义枚举方法，如下来两行代码

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    m.call();
}
```