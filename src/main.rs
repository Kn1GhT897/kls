use std::any::Any;
use std::env;
use std::fs::read_dir;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let result = read_dir(path).unwrap();
    for item in result.into_iter() {
        println!("{:?}", item.unwrap().file_type().unwrap());
    }

}