# hashmap的更新

## 一、概述

HashMap的大小是可以变化，也就是说键值对的数量可以变化，但是每个K同时只能对应一个V。更新HashMap中的数据有以下几种选择

当K存在时

- 替换现有的V
- 保留现有的V，忽略新的V
- 基于现有的V来更新V

当K不存在时

- 添加一对新的K, V

## 二、更新操作

### 2.1 替换现有的V

如果向HashMap插入一对KV，然后再插入同样的K，但是不同的V，那么原来的V会被替换掉，如下示例代码

```Rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue", 10));
    scores.insert(String::from("Blue", 25));

    println!("{:?}", scores);
}
```

### 2.2 只在K不存在任何值的情况下才插入V

`entry`方法，会检查指定的K是否存在一个对应的V，参数为`K`，返回值为 `enum Enrty`，代表是否存在。Entry的`or_insert()`方法，如果K存在，返回对应的V的一个可变引用；如果K不存在，将方法作为K的新值插进去，返回到这个值的可变引用。如下示例代码

```Rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```

### 2.3 基于现有的V来更新V

如下示例代码

```Rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
```

以上代码中，字符串调用`split_whitespace`方法实现按照空格分割成可遍历的单词集合数据，检测`map`集合该单词对应的K，如果不存在则调用`or_insert`方法插入0。最后将返回的值的引用进行 `+1`，即可实现单词计数的效果。

## 三、Hash函数

在默认情况下，HashMap使用了加密功能强大的HashMap函数，Hash函数可以抵抗拒绝服务（Dos）攻击。Hash函数不是可用的最快的Hash算法，但具有更好的安全性。

我们可以指定不同的`hasher`来切换到另一个函数，这里的`hasher`指的是实现了`BuildHasher trait`的类型。
