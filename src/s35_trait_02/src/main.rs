use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 所有Pair<T> 都拥有new函数
impl <T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 实现了Display 和 PartialOrd 的Pair<T> 猜拥有com_display函数
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

fn main(){

}