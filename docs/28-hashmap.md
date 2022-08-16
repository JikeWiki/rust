# hashmap

## 一、概述

HashMap是以键值对的形式存储数据，一个键（key）对应一个值（value），HashMap的内部实现使用了Hash函数，Hash函数决定了如何在内存中存放key和value。HashMap适用场景为：通过K（任何类型）来寻找数据，而不是通过索引。

## 二、创建HashMap

使用`HashMap::new()`函数来创建空的HashMap，再通过`insert()`方法来添加数据。如下示例代码

```rust
use std::collections::HashMap;

fn main() {
    let mut scopes = HashMap::new();

    scopes.insert(String::from("Blue"), 10);
    scopes.insert(String::from("Yellow"), 50);
}
```

HashMap使用比较少，所以不在prelude中。如需要使用HashMap，需要手动引入。标准库对HashMap的支持也比较少，没有内置的宏来创建`HashMap`。HashMap的数据存放在heap内存上。

HashMap是同构的，在一个HashMap中，所有的 K 必须是同一个类型，所有的 V必须是同一个类型。

另一种创建HashMap的方式是通过`collect`方法。在元素类型为`Tuple`的Vector上使用`collect`方法，可以组建一个HashMap。要求Tuple有两个值：一个作为K，一个作为V。`collect`方法可以把数据整合成很多种集合类型，包括HashMap，返回值需要显式指明类型。如下示例代码

```rust
use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    // zip函数有拉链的意思，将两个数组拉到一起，在使用 collect 方法集合数据，最后指定生成的集合数据为 HashMap
    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
}
```

## 三、HashMap和所有权

对于实现了`Copy trait`的类型（例如i32），值会会被复制到`HashMap`中。对于拥有所有权的值（例如String），值会被移动，所有权会转给HashMap。

如果将值的引用插入到HashMap，值本身不会移动，在HashMap有效的期间，被引用的值必须保持有效。

## 四、访问HashMap中的值

使用`get`方法访问HashMap的值，参数为`K`，返回`Option<&V>`，如下示例代码

```Rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    let team_name = String::from("Blue");
    let scores = scores.get(&team_name);

    match scores {
        Some(s) => println!("{}", s),
        None => println!("team not exits"),
    }
}
```

## 五、遍历HashMap

通常使用HashMap的引用进行遍历，因为在遍历之后，我们通常还需要是使用到HashMap，如下示例代码

```Rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    for (k, v) in &scopes {
        println!("{}: {}", K, v);
    }
}
```
