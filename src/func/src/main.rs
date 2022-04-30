// 这是单行注释
fn plus_five(x: i32) -> i32 {
    x + 5
}

/**
 * 这是多行注释
 */
fn main() {
    let x = plus_five(6);
    println!("The value of x is: {}", x);
}
