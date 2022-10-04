use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 生成运动计划
fn simulated_expensive_calculation(intensive: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensive
}

/**
 * 参数1:强度
 * 参数2:随机数
 */
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包，使用一个变量来接收
    let expensitive_closure = |num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups", expensitive_closure(intensity));
        println!("Next, do {} situps!", expensitive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!("Today, run for {} minutes!", expensitive_closure(intensity));
        }
    }
}
