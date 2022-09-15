use std::fmt::Display;

fn longest_with_an_announcement<'a, T>
(x: &'a str, y: &'a str, ann: T) -> &'a str where T:Display {
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let str1 = "abc";
    let str2 = "abcde";

    let data = "initial contents";
    let ann = data.to_string();

    let longest_str = longest_with_an_announcement(str1, str2, ann);

    println!("The longest str is {}", longest_str);
}
