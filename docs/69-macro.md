# 宏

## 一、概述

宏在rust里指的是一组相关特性的集合称谓，包括使用`macro_rules!`构建的声明宏（declarative macro），以及3中过程宏，3种过程宏如下

- 自定义`#[derive]`宏，用于rust或enum，可以为其指定随derive属性添加的代码
- 类似属性的宏，可以在任意条目上添加自定义属性
- 类似函数的宏，看起来像函数调用，对其制定为参数的token进行操作

## 二、函数和宏的差别

本质上，宏是用来编写可以生成其他代码的代码（元编程，metaprogramming）。函数在定义签名时，必须声明参数的个数和类型，而宏可以处理可变的参数。编译器会在解释代码前把展开宏。宏的定义比函数的定义复杂得多，宏时难以阅读、理解、维护。在某个文件调用宏的时候必须提前定义宏或将宏引入当前作用域，而函数可以在任何位置定义并在任何位置使用。

## 三、macro_rules!声明宏

rust中最常见的宏形式：声明宏，类似于match的匹配模式，在定义声明宏时，需要使用`macro_rules!`。如下示例，是一个简化的`vec!`

```rust
// 以下标注意味着它所处的包被引入作用域后才可以使用，缺少下面标注的宏不能引入作用域
#[macro_export]
// 使用macro_rules!声明宏，宏的名称叫vec
macro_rules! vec {
// 大括号里是宏的定义体
// 以下代码类似于match表达式，但是match表达式匹配的是值，这里匹配的是rust的代码结构

// $( $x:expr ) 匹配任何的rust表达式，并命名为$x。
// 后面紧跟的逗号匹配可能出现的逗号分隔符（,）出现在捕获的代码的后面
// 星号（*）代表匹配0个或者多个星号之前的内容
( $( $x:expr ), * ) => {
        {
            // 每次匹配，将会生成类似如下代码
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

那么对于`let v: Vec<u32> = vec![1, 2, 3];`这行代码，将会生成类似如下的代码

```rust
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
```

其实，`macro_rules!`存在一些奇怪的技术细节，所以rust团队正在致力于推出`macro`关键字的第二种声明宏，它和现在宏的工作方式有些类似，但是修复了某些可能的极端情况，而在更新之后，`macro_rules!`就会被标记为弃用。

其实针对宏，大多数程序员只是用，通常不会去编写宏，所以这块我们不再深入研究。

## 四、基于属性来生成代码的过程宏

这种形式的宏更像函数（或者某种形式的过程），它会接收并操作输入的rust代码，再生成另一些rust代码作为结果。一共有三种过程宏，如下

- 自定义派生
- 属性宏
- 函数宏

在创建过程宏时，宏定义必须单独放在它们自己的包中，并使用特殊的包类型。如下示例

```rust
use proc_macro;

// some_attribute是一个用来制定过程宏的占位符 
#[some_attribute]
// 定义过程宏的函数，接收一个TokenStream（TokenStream属于proc_macro包下）
// 产生一个TokenStream作为输出
pub fn some_name(input: TokenStream) -> TokenStream {

}
```

函数附带的属性决定了我们创建的是哪种过程宏，同一个包中可以拥有不同的过程宏。

### 4.1 自定义derive宏

我们通过一个示例简单介绍自定义宏，需求为：创建一个`hello_macro`包，定义一个拥有关联函数`hello_macro`的`HelloMacro` trait，我们需要提供一个自动实现trait的过程宏。

这就使得用户在它们的类型上标注`#[derive(HelloMacro)]`，进而得到`hello_macro`的默认实现。

实现过程比较复杂，代码实现过程暂时留空......

### 4.2 类似属性的宏

类似属性的宏可以叫属性宏，属性宏与自定义 derive 宏类似，它允许创建新的属性，但不是为 derive 属性生成代码。属性宏更加灵活，derive 只能用于struct和enum，而属性宏可以用于任意条目，例如函数。

下面是一个不完整的示例代码

```rust
// 在MVC的项目中，Controller通常存在这样的属性，用来做路由
// 而route使用一个共同宏来定义
#[route(GET, "/")]
fn index() {}

#[proc_macro_attribute]
// 宏定义的函数签名如下，有两个TokenStream作为参数
// 前面的attr: TokenStream对应“GET, "/"”，后面的item: TokenStream对应index函数体
// 最后返回一个TokenStream
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
```

### 4.3 类似函数的宏

类似函数的宏又叫函数宏，函数宏的定义类似于函数调用的宏，但比普通的函数更加灵活。函数宏可以接受TokenStream作为参数，与另外两种宏的过程一样，在定义中使用rust代码来操作TokenStream。

下面是一个不完整的示例代码

```rust
// 我们定义一个能够解析SQL语句的宏
let sql = sql!(SELECT * FROM posts WHERE id = 1);

#[proc_macro]
// 函数的签名如下，接受一个TokenStream，再返回相应功能的TokenStream
pub fn sql(input: TokenStream) -> TokenStream {}
```
