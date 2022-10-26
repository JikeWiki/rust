pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // 下面一行代码表示Box里的元素都实现了Draw trait
    // 所以只要实现了Draw trait的数据，都可以防汛Vec中
    // 之所以不使用泛型，应为泛型里只能存放一种元素
    pub commponents: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        // 遍历所有组件，执行draw方法
        for component in self.commponents.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制一个按钮
    }
}
