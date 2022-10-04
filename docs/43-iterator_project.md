# 使用迭代器优化IO项目

在前面两节，我们开发了一下给予IO操作的示例项目，我们现在来优化之前的项目。本节对应的代码目录`src/s43_minigrep_optmize`。

## 一、使用迭代器优化Config结构体方法

回顾之前的一个代码片段，在Config的new函数中包含如下代码

```Rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        // CASE_SENSITIVE 环境变量出现则代表区分大小写，而并不关心起具体值
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

在去参数的时候，之所以使用`clone`方法，是因为new函数并不拥有`args`参数的所有权，传进来仅仅只字符串切片，而返回的Config需要拥有字符串的所有权。针对`query`和`filename`两个字段，必须克隆出一份，这样Config才拥有它两的所有权。

而迭代器有用数据的所有权，这样就不需要使用切片了，我门还可以使用迭代器自带的功能进行长度检查和索引。在`main`函数中，`env::args()`产生了个迭代器，我们可以直接将迭代器传给new函数。改造如下

```Rust
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }

        args.next(); // 放出第一个元素

        // 取第二个元素
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // 取第三个元素
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // CASE_SENSITIVE 环境变量出现则代表区分大小写，而并不关心起具体值
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

## 二、使用迭代器优化搜索函数

我们还可以优化`search`方法，使用迭代器替换原来的逻辑，如下示例代码

```Rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

使用迭代器的好处是让开发者能专注于高层的业务逻辑，而不必陷入写循环、维护临时变量的细节工作里。

`search_case_insensitive`函数也可以优化成如下示例代码

```Rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}
```
