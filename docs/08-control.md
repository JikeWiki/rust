# 控制流

## 1. if和else表达式

### 1.1. if-else表达式

if表达式允许你使用条件来执行不同的代码分支，这个条件必须是 bool类型。if表达式中，与条件相关量的代码叫做分支（arm），可选的在在if表达式之后加else表达式。如下示例代码

```rust
fn main() {
    let num = 3;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

### 1.2. 使用else if处理多重条件

当逻辑条件比较多时，我们可以使用else if处理多重条件，如下示例代码

```rust
fn main() {
    let number = 6;
    if number % 4 == 0 {
        print!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        print!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        print!("{} is divisible by 2", number);
    } else {
        print!("{} is not divisible by 4, 3, or 2", number);
    }
}
```

但如果程序里使用了多于一个`else if`，那么最好使用`match`来重构代码。

### 1.3. 在let语句中使用if

因为if是一个表达式，所以可以将它放在let语句中等号右边，如下示例

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    print!("The value of number is: {}", number);
}
```

单要注意的是，if表达式和紧跟着的else表达式，返回的结果必须是一种类型，否则rust将会编译报错。rust在编译时会进行相应的安全检查，编译的时候就必须明确变量的类型是什么，以便在其他地方使用。如果我们将上面的第三行代码改为如下内容，此时if和else返回值的类型是不同的，也就是说在编译时无法确定`number`变量的具体类型，将会编译不通过

```rust
let number = if condition { 5 } else { "6" };
```

## 2. rust的循环

rust提供了3中循环：loop、while、for

### 2.1. loop循环

loop关键字告诉rust反复执行一块代码，知道逻辑中主动让其停止。可以在loop循环中使用break关键字来告诉程序合适停止循环。如下示例代码

```rust
fn main() {
    let mut counter = 0;

    let rustlt = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", rustlt);
}
```

### 2.2. while条件循环

另一种常见的循环模式时每次执行循环之前都判断一次，while条件循环就是为这种模式而生

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

### 2.3. 使用for循环来遍历集合

可以使用while循环或者loop来遍历集合，但是容易出错而且效率低，如下例子

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
```

上面的程序具有以下缺点

- **容易出错**：我们在 while的执行条件中使用了`index < 5`表达式，这里的5代表数组索引的数量，但如果我们数组元素很多，就会很容易写错索引的数量。

- **执行效率低**：因为执行每次遍历之前，程序先会去检查`index < 5`这个条件。

针对集合的遍历场景，使用for循环更加简洁紧凑，它可以针对集合中的每一个元素执行一些代码。由于for循环的安全性、简洁性，所以它在rust里用的最多。如下代码：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

for循环具有一下优点

- 永远不会出现索引错误
- 每次循环都不需要额外进行判断

### 2.4. Range

Range由标准库提供，指定一个开始数字和一个结束数字（不含结束数字），Range可以生成它们之间的数字。rev方法可以反转Range。如果能结合  for 循环和 Range 来实现 while 循环的功能，将会在一定程度上提高程序的性能。如下示例代码

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

上面的程序，将会输出3到1的结果。
