# 如何编写测试

## 一、什么是测试

在rust里，一个测试就是一个函数，他被用于测试其他的非测试代码的功能是否和预期一致。所以在测试的函数体里，通常会执行3个操作，如下

- 准备数据/状态
- 运行被测试的代码
- 断言（Assert）结构

## 二、解剖测试函数

测试函数需要使用test属性（attribute）进行标注，`Attribute`就是一段Rust代码的元数据，它只是对代码进行修饰，不会对代码原有的逻辑进行修改。在函数上加`#[test]`，可以把函数变成测试函数。

当我们编写完成测试函数之后，使用`cargo test`命令运行所有的测试。运行过程中，Rust会构建一个Test Runner可执行文件，他会逐个运行标注了 test 的函数，并报告其运行是否成功。

当我们使用cargo创建library项目的时候，会生成一个 test module，里面有一个 test 函数。我们可以参照它来编写其他的测试函数，实际上，我们可以添加任意数量的test module或test函数。

## 三、测试示例

我们使用`cargo new adder --lib`即创建了一个库项目，可以看到默认生成的代码`lib.rs`如下

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

`it_works`这个函数是测试函数，是因为该函数上面加了`#[test]`注释，并不是因为它在test模块中，test模块中还可以拥有普通的（非测试）函数。`assert_eq!`是一个断言宏。使用`cargo test`执行测试过程。

## 四、测试失败

符合测试失败扽条件如下

- 测试函数出现panic就表示测试失败
- 每个测试运行在一个新线程，当主线程看到某线程挂掉，那个测试则标记为失败

## 五、断言（Asert）

### 5.1 使用`assert!`宏检查测试结果

`assert!`宏，来自标准库，用来确定某个状态是否为`true`。`assert!`宏接受一个布尔类型的参数，当参数为true时，测试通过，当参数为false时，则调用panic，测试失败。下面我们编写一个例子

首先定义一个矩形，如下

```Rust
// 定义一个矩形
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

// 定义一个函数，用于判断该矩形是否能容纳另一个矩形
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

我们在同一个文件内添加测试模块，如下代码

```Rust
#[cfg(test)]
mod test {
    // 以下代码表示导入外部模块所有内容
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {

        // 准备数据：声明两个矩形
        let larger = Rectangle {
            length: 8,
            width:7 ,
        };

        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        // 执行被测试代码
        assert!(larger.can_hold(&smaller));
    }
}
```

### 5.2 使用assert_eq!和assert_ne!测试相等性

这两个宏都来自标准库，用于判断两个参数是否相等或不等。实际上，他们使用的就是`==`和`!=`运算符。

如果断言失败，将自动打印出两个参数的值。使用debug格式打印参数，要求参数实现了`PartialEq`和`Debug` Traits（所有的基本类型和标准库里大部分类型已实现了这两个Trait）。对于自定的结构体和枚举来说，需要自行实现这两个Trait。

下面我们写一个加2的函数，并使用测试代码进行测试，如下

```Rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

在rust里，用于断言是否相等的宏，第一个参数可以是期待的值，第二个参数可以是计算的值。他们可以对换过来，即第一个参数是计算的值，第二个参数是期待的值。所以我们把rust断言是否相等的宏的第一个参数叫做`坐值`，第二个参数叫做`右值`。

## 六、自定义的错误信息

可以向`assert!`、`assert_eq!`、`assert_ne!`添加可选的自定义错误信息。如果添加了自定的错误信息，这些自定义的错误信息和失败消息都会打印出来。

`assert!`: 第1个参数必填，自定义消息作为第2个参数。

`assert_eq!`和`assert_ne!`: 前2个参数必填，自定义消息作为第3个参数。

自定义消息参数会被传递给`format!`宏，可以使用`{}`占位符。下面是一个示例

```Rust
pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let  result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting didn't contain name, value was '{}'", result);
    }
}
```

以上的测试将会失败，并且提示出我们的自定义错误信息。

## 七、验证错误处理的情况

### 7.1 使用should_panic更精确

测试出了验证代码的返回值是否正确，还需验证代码是否如预期的处理了发生错误的情况。如我们可以验证代码在特定情况下是否发生了`panic`。此时我们需要使用`should_panic`属性，如果使用了该属性，那么函数如果发生panic则测试通过，函数如果没有panic则测试失败。如果示例

```Rust
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

使用`should_panic`标注了测试函数应该出现panic，实际上函数也出现了panic，所以测试通过。

### 7.2 让should_panic更精确

为should_panic属性添加一个可选的`expected`参数，这样的话测试过程将会检查失败的消息中是否包含所指定的文字。如下示例

```Rust
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic("Guess value must be between 1 and 100, got {}.", value);
        } else if value > 100 {
            panic("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

在上面的示例中，将会执行到`value > 100`的逻辑产生panic，并且panic的错误信息完全包含了`expected`参数，所以测试通过。

## 八、在测试中使用Result<T, E>

到目前为止，测试运行失败，都是因为panic，但导致测试失败，不仅仅是因为panic，还可以使用`Result<T, E>`枚举来达到同样目的。即在编写测试的时候无需panic，可以使用`Result <T, E>`作为返回类型编写测试。如果返回`OK`，测试通过，如果返回`Err`则测试失败。如下例子

```Rust
#[cfg(test)]
mod tests {
    #[test]

    fn it_works -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

在以上代码中，将会执行`2 + 2 == 4`的逻辑，所以测试通过。需要注意的是，不要在`Result<T, E>`编写的测试上标注`#[should_panic]`。因为在运行失败的时候，会执行返回`Err`，不回发生panic。
