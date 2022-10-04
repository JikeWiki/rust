use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T> 
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/**
 * 参数1:强度
 * 参数2:随机数
 */
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包，使用一个变量来接收
    let mut expensitive_closure = Cacher::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });



    if intensity < 25 {
        println!("Today, do {} pushups", expensitive_closure.value(intensity));
        println!("Next, do {} situps!", expensitive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!("Today, run for {} minutes!", expensitive_closure.value(intensity));
        }
    }
}
