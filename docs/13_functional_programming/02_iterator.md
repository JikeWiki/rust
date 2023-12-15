# 迭代器

## 一、什么是迭代器？

迭代器模式是对一系列项执行某些任务。在这个过程中迭代器负责遍历每个项目，并确定序列（遍历）何时完成。

Rust里的迭代器是惰性的，除非调用消费迭代器的方法，否则迭代器本身没有任何效果。可以这么理解：迭代器在不使用它的时候，它什么都不做。当我们使用了某些可以消耗迭代器的方法的时候，这时候迭代器才真正起到了迭代器的作用。我们先来看个示例：

```Rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
}
```

上面的示例代码中，v1使用调用iter方法得到了一个迭代器，但是迭代器没有使用，那么迭代器不产生任何效果。我们可以对迭代器进行遍历，如下示例代码

```Rust
for val in v1_iter {
    println!("Got: {}", val);
}
```

上面的代码相当于迭代器里的每一个元素用在了循环里，具体的用户就是把它打印出来。

## 二、Iterator trait和next方法

所有的迭代器都实现了 Iterator trait，其定义于标准库，大致如下

```Rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

type Item和Self::Item定义了与此trait的关联类型（前面还没提到关联类型）。现在我们只需要知道：实现Iterator trait需要你定义一个Item类型，它用于next方法的返回类型（迭代器的返回类型）。

所以Iterator Trait仅要求实现一个方法：next。next方法每次返回迭代器中的一项，即迭代器中的一个元素，返回的结果包裹在`Some`里，迭代结束的时候，返回`None`。在实际使用中，我们可以直接调用next方法。如下示例的测试代码

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }
}
```

## 三、几个迭代方法

迭代器存在几个迭代方法，分别如下

- iter方法：在不可变引用（指的是元素不可变引用，并不是迭代器本身）上创建迭代器。

- into_iter方法：创建迭代器会获得所有权。

- iter_mut方法：迭代可变引用。

## 四、消耗和产生迭代器的方法

### 4.1 消耗型迭代器

在标准库中，Iterator trait有一些默认实现的方法，其中有一部分方法会调用next方法。所以如果我们想实现 Iterator trait时就必须实现next方法。我们把调用next的方法叫做“消耗型适配器”，最终调用next方法的过程会把迭代器耗尽。

例如：sum方法会消耗迭代器，并且取得迭代器的所有权，sum方法就是反复调用next来编译元素，每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和。如下示例的测试代码

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
```

### 4.2 产生其他迭代器的方法

定义在 Iterator trait 上的另一个方法叫做“迭代器适配器”，它们的作用是把迭代器转换为不同类型的迭代器。可以通过链式调用使用多个迭代器来执行复杂的操作，这种调用可读性高。

例如：map方法，接收一个闭包作为参数，这个闭包作用域迭代器的每个元素，他把当前迭代器的元素转化为另外一个元素，然后这些另外的元素就组成了一个新的迭代器。但要注意的是，迭代器是惰性的，如果不消耗它们，他们将什么都不会操作，如下示例的测试代码

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        v1.iter().map(|x| x+1);
    }
}
```

上面的代码中，并不会对v1中的3个元素进行加1操作。如果我们此时调用一个消耗型的`collect`方法，它会把所有的结果收集到一个某个数据类型的集合里。至于是收集的集合是什么类型，我们使用`<_>`标注让rust推断即可，如下示例代码

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}
```

### 4.3 总结

通过“4.1”、“4.2”中的示例，总结如下

- “消耗型适配器”方法：sum、collect
- “迭代器适配器”方法：map

## 五、使用闭包捕获环境

我们可以通过`filter`方法来捕获环境，该方法接收一个闭包作为参数。这个闭包在遍历迭代器的每个元素时，都会返回bool类型。如果闭包返回true，则当前元素将会包含在filter产生的迭代器中；如果闭包返回false，当前元素将不会包含在filter产生的迭代器中。如下示例代码

```Rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[test]
fn filter_bu_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("hoot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("hoot"),
            },
        ]    
    );
}
```

## 六、创建自定义的迭代器

使用Iterator trait来创建自定义迭代器，我们只需要执行一步操作，实现next方法即可。我们下面创建一个迭代器，该迭代器能从1遍历到5，如下示例代码

```rust
struct Counter {
    counter: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { counter: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < 5 {
            self.counter += 1;
            Some(self.counter)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

接下来我们来扩展一下，使用其他的Iterator的方法。上面的迭代器，从1迭代到5，我们想让这样的两个迭代器，他们的每队元素进行相乘。第一个迭代器元素从1到5，第二个迭代器从2开始，到5结束。产生新的迭代器必须能够被3整除，所以需要跑过滤，过滤之后，我们把剩余的元素求和并返回。实现逻辑如下代码

```Rust
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
```
