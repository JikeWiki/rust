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
