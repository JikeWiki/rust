# 不安全Rust

## 一、概述

rust隐藏着第二个语言，它没有强制内存安全保证，这就是 unsafe rust（不安全的rust）。实际上和普通的rust一样，但提供了额外的“超能力”。Unsafe Rust存在的原因如下

第一个原因是，静态分析是保守的，编译器在判断一段是否拥有安全保证时，它宁可错杀一些合法的程序，也不会接受仅仅是可能存在非法的代码，金尽管这些代码是安全的。而使用 Unsafe Rust，我们就是告诉编译器，我知道自己在做什么，我愿意自己承担这个风险。

第二个原因是，计算机硬件本身就是不安全的，Rust 需要能够进行底层系统编程，如果不允许 Unsafe Rust，那么这些工作就无法完成了。

## 二、Unsafe超能力

我们可以使用`unsafe`关键字来切换代码到 Unsafe Rust，开启一个代码块，里面存放着 unsafe 代码。Unsafe Rust 里可以执行四类动作（unsafe 超能力），如下

- 解引用原始指针
- 调用unsafe函数或方法
- 访问或修改可变的静态变量
- 实现unsafe trait

需要注意的是，unsafe 并没有关闭借用检查或停用其他安全检查。任何内存安全相关的错误，我们必须留在unsafe块里。同时，我们应该尽可能隔离 unsafe 代码，最好将其封装在安全的抽象里，提供安全的API。

### 2.1 解引用原始指针

在不安全的rust里，有两种类似于引用的新型指针，它们就叫做原始指针或者裸指针。原始指针的写法如下

- 可变的：`*mut T`
- 不可变的：`*const T`。意味着指针在解引用后不能直接对其进行赋值。

需要注意的是，这里的`*`不是解引用符号，它是类型名的一部分。与引用不同，原始指针允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则。原始指针无法保证指向合理的内存，原始指针允许为`null`，原始指针也不实现任何的自动清理。

在放弃保证的安全，rust换取了更好的性能，以及与其他语言或硬件接口的能力。下面我们来看一下创建原始指针的例子

```rust
fn main() {
    let mut num = 5;

    // 以下两个指针来自有效的引用，所以以下两个指针也是有效的
    // 不可变的原始指针
    let r1 = &num as *const i32;
    // 可变的原始指针
    let r2 = &mut num as *const i32;

    // 创建一个无法确定其有效性的指针
    let address = 0x012345usize;// 内存地址中可能有数据，可能没有数据
    let r = address as *const i32;// 创建未确定有效性的内存的原始指针
}
```

我们可以在不安全代码之外创建原始指针，但是只能在不安全代码中对其进行解引用。如下示例代码

```rust
fn main() {
    let mut num = 5;

    // 以下两个指针来自有效的引用，所以以下两个指针也是有效的
    // 不可变的原始指针
    let r1 = &num as *const i32;
    // 可变的原始指针
    let r2 = &mut num as *const i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    // 创建一个无法确定其有效性的指针
    let address = 0x012345usize;// 内存地址中可能有数据，可能没有数据
    let r = address as *const i32;// 创建未确定有效性的内存的原始指针
    unsafe {
        println!("r: {}", *r);
    }
}
```

在以上代码中，程序运行到第二个unsafe代码块时，发生了错误。因为我们指定的地址是有可能不存在的，如果出现错误，也有由我们自己来负责。

为什么要使用原始指针？目的是与C语言进行接口交互，另外可以构建借用检查器无法理解的安全抽象。

### 2.2 调用unsafe函数或方法

unsafe函数或方法指的是在函数或方法前加上了`unsafe`关键字，除此之外和其他函数或者方法没什么区别。但是在调用这种函数或者方法之前，需要手动满足一些条件，因为rust无法对这些条件进行验证。另外，想要调用`unsafe`的函数或者方法，必须在`unsafe`代码块里进行调用。如下示例

```rust
unsafe fn dangerous() {}

fn mian() {
    unsafe {
        dangerous();
    }
}
```

### 2.3 创建unsafe代码的安全抽象

函数包含unsafe代码并不需要将整个函数标记为unsafe，将unsafe代码包裹在安全函数中是一个常见的抽象。如下示例代码

```rust
use std::{slice, vec};

// 不安全代码的安全抽象
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

### 2.4 使用extern函数调用外部代码

extern关键字：简化创建和使用外部函数接口（FFI）的过程。外部函数接口（FFI，Foreign Function Interface），它允许一种编程语言定义函数，并让其他编程语言能调用这些函数。任何在`extern`块里声明的函数都是不安全的，因为其他语言并不会强制执行rust遵守的规则，而rust又无法对它们进行检查。所以在调用外部函数的过程中，如何保证程序安全的责任也落在了开发者身上。如下示例代码

```rust
// C 指明了外部函数使用的应用二进制接口（Application Binary Intreface），即ABI
// ABI 是用于定义函数在汇编层面的调用方式
extern "C" {
    // 想要调用的外部函数的名称和签名
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

"C" ABI是最常见的ABI，它遵循C语言的ABI。

### 2.5 从其他语言调用rust函数

可以使用`extern`创建接口，其他语言通过它们可以调用rust函数。在`fn`前添加`extern`关键字，并制定对应的ABI。还需要添加`#[no_mangle]`注解，目的是避免rust在编译时改变它们的名称。如下示例

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C");
}

fn main() {}
```

`call_from_c`函数在编译和链接之后就可以被C语言访问。

### 2.6 访问或修改一个可变静态变量

rust支持全局变量，但因为所有权机制可能产生某些问题，例如数据竞争。在rust里，全局变量叫做静态（static）变量，如下示例代码

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

静态变量与常量类似，命名规范示例如：`SCREAMING_SNAKE_CASE`。声明的时候必须标注类型，静态变量只能存储`'static`声明周期的引用，无需显式标注。访问不可变的静态变量是安全的。常量与静态变量的区别如下

- 静态变量：有固定的内存地址，使用它的值总会访问同样的数据
- 常量：允许使用它们的时候对数据进行赋值

静态变量是可变的，访问和修改静态可变变量是不安全（unsafe）的操作。如下示例代码

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 修改静态变量
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    // 访问静态变量
    unsafe {
        println("COUNTER: {}", COUNTER);
    }
}
```

### 2.7 实现不安全的trait

当某个trait中存在至少一个方法拥有编译器无法校验的安全因素时，就称这个trait时不安全的，声明unsafe trait，需定义前加`unsafe`关键字。该trait只能在unsafe代码块中实现。如下示例代码

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implements go here
}

fn main() {}
```

## 三、何时使用unsafe

使用unsafe代码的时候要考虑以下问题

- 使用unsafe代码时编译器无法保证内存安全，让开发者保证unsafe代码正确性并不简单。但有充足的理由使用unsafe代码时，就可以这样做
- 通过显示标记`unsafe`，可以在出现问题时轻松定位
