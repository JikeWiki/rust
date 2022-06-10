fn main() {
    let mut s = String::from("Hello world");
    let wordIndex = first_world(&s);

    s.clear();
    print!("{}", wordIndex)
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
