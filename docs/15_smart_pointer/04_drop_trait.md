# Drop Trait

## 一、概述

当一个数据类型实现Drop Trait之后，可以让我们自定义当值离开作用域时发生的动作。通常发生的动作包括：文件、网络资源的释放等。任何类型都可以实现Drop trait，Drop Trait只要求实现`drop`方法，`drop`的参数是对self的可变引用。Drop trait在预导入模块（prelude）里。我们先看一个示例代码

```rust
struct CustomartPointer {
    data: String,
}

impl Drop for CustomartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` !", self.data)
    }
}

fn main() {
    let c = CustomartPointer { data: String::from("my  stuff") };
    let d = CustomartPointer { data: String::from("other  stuff") };

    println!("CusomSmartPointers created")
}
```

执行程序之后，输出以下内容

```bash
CusomSmartPointers created
Dropping CustomSmartPointer with data `other  stuff` !
Dropping CustomSmartPointer with data `my  stuff` !
```

这表明，两个变量离开作用域的时候调用了`drop`方法。

## 二、使用std::mem::drop来提前drop值

我们很难直接自动调用drop功能，其实也没有必要，因为Drop trait的目的就是进行自动的释放逻辑处理。此外，rust也不允许手动调用Drop trait的`drop`方法，但可以调用标准库的`std::mem::drop`函数，该函数也在预导入模块中，来提前drop值，相当于提前调用了Drop trait的`drop`方法。如下示例代码

```rust
let c = CustomartPointer { data: String::from("my  stuff") };
drop(c);
let d = CustomartPointer { data: String::from("other  stuff") };

println!("CusomSmartPointers created")
```

这时候，输入内容如下

```bash
Dropping CustomSmartPointer with data `my  stuff` !
CusomSmartPointers created
Dropping CustomSmartPointer with data `other  stuff` !
```

这时候，你可能会有疑问：提前调用`drop`函数，那么会不会出现重复释放（double free）的错误呢？答案是不会的，rust的设计很安全，它的所有权系统会保证引用的有效，而`drop`方法也只会在不再使用该值的时候只调用一次。
