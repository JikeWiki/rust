# 高级函数和闭包

## 一、函数指针

在rust中，闭包可以传递给函数，还可以将函数传递给其他函数。在传递的过程中会被强制转换成fn类型，fn类型就是“函数指针”（function pointer）。如下

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

## 二、函数指针与闭包的不同

闭包实现了三种闭包trait，而函数指针即fn是一个类型，不是一个trait。我们可以直接制定fn为参数类型，不用声明一个以内`Fn trait`为约束的泛型参数。

函数指针实现了3种闭包trait（Fn, FnMut, FnOnce），所以总是可以把函数指针用作参数传递给一个接收闭包的函数。也正是因为这个原因，我们更倾向于搭配闭包trait的泛型来编写函数，这样这个函数可以同时接收闭包和普通函数作为参数。

某些情景下，我们只想接收fn而不接收闭包，如与外部不支持闭包的代码交互：C函数。我们先看一个示例，将vec的每一个元素转换为String，如下代码

```rust
fn main() {
    // 使用闭包
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();

    // 使用函数
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
}
```

我们再看一个示例，在创建一些枚举数据时，需传入一个数据，并返回一个值。我们可以把这种构造器也作为实现了闭包trait的函数指针来进行使用，如下示例代码

```rust
fn main() {
    enum Status {
        Value(u32),
        Stop,
    }

    let v = Status::Value(2);

    let list_of_statuses: Vec<Status> = 
    (0u32..20)
    .map(Status::Value)
    .collect();
}
```

## 三、返回闭包

闭包使用trait进行表达，无法在函数中直接返回一个闭包，因为rust无法推断出需要多少空间来存储一个返回的闭包，但是可以将一个实现了该trait的具体类型作为返回值。如下示例

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {}
```
