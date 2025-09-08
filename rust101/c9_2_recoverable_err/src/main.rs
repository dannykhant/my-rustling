use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_res = File::open("hello.txt");

    let greeting_file = match file_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("Problem creating file... err: {error:?}")
            }
            _ => panic!("Problem opening file... err: {error:?}")
        }
    };
}
