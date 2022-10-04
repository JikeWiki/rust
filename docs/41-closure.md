# 闭包

## 一、什么是闭包

闭包：可以捕获其所在环境的匿名函数。闭包具体以下特性

- 闭包是个匿名函数
- 可以保存为变量，或者作为参数传给另外一个函数，或者作为另外一个的返回值
- 可以在某个地方创建闭包，然后在另一个上下文调用闭包来完成运算
- 可从其定义的作用域捕获值

## 二、生成自定义运动计划程序

下面有一个程序示例，该程序的算法逻辑我们不关心，重点是算法中的计算过程需要几秒钟的时间。而我们优化的目标是不让用户发生不必要的等待，仅在必要时调用该算法，只调用一次。程序如下

```Rust
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 生成运动计划
fn simulated_expensive_calculation(intensive: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensive
}

/**
 * 参数1:强度
 * 参数2:随机数
 */
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        // 在以下代码中，调用了两次耗时函数，可以优化
        println!(
            "Today, do {} pushups",
            simulated_expensive_calculation(intensity)
        );

        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

我们先进行第一测优化，优化后的`generate_workout`函数如下

```Rust
fn generate_workout(intensity: u32, random_number: u32) {

    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result
        );

        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

此时，针对`intensity < 25`的情况，耗时的函数不再需要调用两次，调用是在`if`判断之前执行的。但是此时也引入了一个其他的问题，在`else`的逻辑里，当随机数等于3时，是不需要调用耗时函数的。而现在的写法是无论任何情况下都会调用函数，这显得有点浪费了。

## 三、使用闭包

而我们真正希望的是函数定义在一个地方，在需要结果的时候才执行相关代码，这正是闭包的用武之地。下面我们使用闭包来解决该问题。使用闭包优化之后的`generate_workout`函数如下

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包，使用一个变量来接收
    let expensitive_closure = |num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups", expensitive_closure(intensity));
        println!("Next, do {} situps!", expensitive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!("Today, run for {} minutes!", expensitive_closure(intensity));
        }
    }
}
```
