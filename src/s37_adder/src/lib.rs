#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod test {
    // 以下代码表示导入外部模块所有内容
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {

        // 准备数据：声明两个矩形
        let larger = Rectangle {
            length: 8,
            width:7 ,
        };

        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        // 执行被测试代码
        assert!(larger.can_hold(&smaller));
    }
}