# use

## 一、概述

可以使用use关键字将路径导入到当前作用域内，引入的内容仍然遵循私有性规则，也就是说只有公共的模块引入进来才可以使用。如下示例代码

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function();
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

在上面的示例代码中，虽然我们引入了`hosting`模块，但是仍然不能调用 `some_function` 这个私有函数。

也可以使用相对路径的方式引入模块，如下示例代码

```rust
use front_of_house::hosting;
```

在引入代码的时候，我们通常引用到父级模块即可，再通过父级模块调用函数，而不是引入到具体的函数，这样我们就能容易的区分函数是外部引入的，还是本模块就存在的。

## 二、use的管用做法

### 2.1 函数

将函数的父级模块引入作用域，指定到其父级即可；

- struct、enum、其他

对于非函数条目，则指定完整路径，即指定到其本身。如下示例代码

```Rust
use std::collection::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

### 2.2 同名条目

如果在不同的模块存在两个同名条目，而在一个源码文件中需要同时使用这两个模块，这时候use引用指定到父级即可。如我们需要`std::fmt::Result`和`std::io::Result`这两个Result，则需要将其父模块同时引入，如下示例代码

```rust
use std::fmt;
use std::io;

fn f1() -> fmt::Result {
    // 函数内容，此处省略
}

fn f2() -> io::Result {
    // 函数内容，此处省略
}
```

除此，我们还可以使用`as`关键字解决该问题，as 关键字可以为引入的路径指定本地的别名，如下示例代码

```rust
use std::fmt::Result;
use std::io::Result as IOResult;

fn f1() -> Result {
    // 函数内容省略
}

fn f2() -> IOResult {
    // 省略函数体
}
```

### 2.3 使用pub use重新导出名称

使用use将路径（名称）导入到作用域后，该名称在此作用域是私有的。如果我们使用`pub use`关键字进行引用，那么在外部就可以使用，如下示例代码

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function();
    }
}

pub use crate::front_of_house::hosting;
```

`pub use`将条目引入到作用域内，同时该条目可以被外部代码引入到它们的作用域内。

## 三、使用外部包（package）

`Cargo.toml`添加依赖的包（package）的声明，当我们运行`cargo build`时，cargo将从<https://crate.io>添加对应包到本地，下载好之后，使用`use`关键字将特定条目引入到作用域。

不过使用使用原网站下载依赖可能会很慢，因为<https://crate.io>的服务器在国外，我们可以使用修改文件文件将下载的源指定改为国内的cargo镜像地址，修改文件`~/.cargo/config`，添加以下内容

```ini
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

以上的配置指定了当cargo去拉去包依赖的时候，将使用清华镜像站。

标准库`std`也被当作外部包，不需要修改`Cargo.toml`来包含`std`。不过在需要使用具体条目的时候，还是需要使用`use`关键字将`std`中特定的条目引入到当前作用域。

## 四、使用嵌套路径清理大量的的use语句

如果使用同一个包或者模块下的多个条目，则可以使用嵌套路径在同一行内将上述条目进行引入，引入格式如下

```Rust
路径相同的部分::{路径差异的部分}
```

如

```rust
use std::cmp::Ordering;
use std::io;
```

可以改为

```Rust
std std::{
    cmp::Ordering,
    io
}
```

如

```Rust
use std::io;
use std::io::Write;
```

可以改为

```Rust
use std::io::{
    self,
    Write
}
```

`*`代表通配符号，我们还可以使用`*`把路径中所有的公共条目都引入到当前的作用域。如下示例代码

```rust
use std::collections::*;
```

但这种引入方式需要谨慎使用，适合以下应用场景

- 测试。将所有被测试代码引入到tests模块中
- 有时被用于预导入（prelude）模块

## 五、将模块内容移动到其他文件

定义模块时，如果模块后面是“;”，而不是代码块，Rust会从模块同名的文件中加在内容，模块树的结构不会变化。随着模块逐渐变大，该技术让你可以把模块的内容移动到其他文件中。
