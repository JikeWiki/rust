fn main() {
    let s1 = String::from("hello");

    // 在main函数里将s1的所有权移动给calculate_length函数，在 calculate_length 再将所有权移交回来
    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    // calculate_length 函数再将s的所有权移动给 调用它的函数
    (s, length)
}
