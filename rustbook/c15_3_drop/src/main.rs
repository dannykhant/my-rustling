struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("hey")
    };
    println!("--");
    drop(c);
    println!("--");
    let d = CustomSmartPointer {
        data: String::from("june")
    };
}
