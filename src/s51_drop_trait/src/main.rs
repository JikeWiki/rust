struct CustomartPointer {
    data: String,
}

impl Drop for CustomartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` !", self.data)
    }
}

fn main() {
    let c = CustomartPointer {
        data: String::from("my  stuff"),
    };
    drop(c);
    let d = CustomartPointer {
        data: String::from("other  stuff"),
    };

    println!("CusomSmartPointers created")
}
