#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    // area 函数借用了area的实例，调用完成之后主函数仍然可以调用reat
    println!("{}", area(&rect));

    println!("{:?}", rect)
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
