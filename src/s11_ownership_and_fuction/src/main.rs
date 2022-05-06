fn main() {
    let r = dangling();
}

fn dangling() -> &String {
    let s = String::from("hello");
    &s
}
