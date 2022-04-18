use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是{}", secret_number);

    println!("猜一个数字");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜的数字是: {}", guess);

    // 先将guess变量转换为整数类型
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => print!("To big!"),
        Ordering::Equal => println!("You win"),
    }
}
