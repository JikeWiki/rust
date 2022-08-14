# 路径

## 一、概述

在rust里，如果我们想要找到某个模块，我们必须知道其路径，rust的路径有两种形式。

绝对路径：从crate root开始，使用 crate 名或字面值 crate。相对路径：从当前模块开始，使用self、super，或者当前模块的标识符。不干事绝对路径还是相对路径，只有由一个表示符组成，如果存在多个标识符，标识符之间使用`::`进行分割。

示例代码如下`src/lib.rs`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 使用绝对路径调用函数
    crate::front_of_house::hosting::add_to_waitlist();
    // 使用相对路径调用函数
    front_of_house::hosting::add_to_waitlist();
}
```

以上的代码分别使用绝对路径和相对路径调用函数，`front_of_house`模块相对`eat_at_restaurant`函数在同一个级别，所以函数里可以直接调用。不过上面的代码看似没有问题，但当我们编译的时候会报错。我们将在下文解决这个问题。

## 二、私有边界（privacy boundary）

模块不仅可以组织代码，还可以定义私有边界。如果想把函数或者struct等设置为私有，就可以将它放到某个模块中。Rust中所有条目（函数、方法、struct、enum、模块、常量）默认是私有的。父级模块无法访问子级模块中的私有条目，在子模块里，可以使用所有祖先模块的条目。

### 2.1 pub关键字

使用`pub`关键字可以将某些条目标记为公共的。我们将“一”中的代码改为如下，将不会再出现错误

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 使用绝对路径调用函数
    crate::front_of_house::hosting::add_to_waitlist();
    // 使用相对路径调用函数
    front_of_house::hosting::add_to_waitlist();
}
```

`front_of_house`之所以不需要添加`pub`关键字，因为`front_of_house`和`eat_at_restaurant`都在文件根级，可以互相调用。

### 2.2 super关键字

`super`关键字用来访问父级模块中的内容，类型文件系统中的`..`。如下示例代码

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用相对路径的方式调用
        super::serve_order();
        // 使用绝对路径的方式调用
        crate::serve_order();
    }

    fn cook_order() {}
}
```

### 2.3 pub struct

pub 放在struct 之前，代表公共的结构体，struct 里面的字段默认是私有的，想把哪个字段设置为公有，就在对应的字段前面加上pub关键字。如下示例代码

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peachers"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
```

### 2.4 pub enum

pub 放在 enum 前面是将enum声明为公共。枚举中的变体都变成公共的。如下示例代码

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```
