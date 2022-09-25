use std::fs::{DirEntry};
pub struct Output;

impl Output {
    fn print_file(file: String) {
        print!("{}\t", file);
    }

    fn print_dir(dir: String) {
        use colored::*;
        print!("{}\t", dir.cyan());
    }

    fn print_symlink(symlink: String) {
        use colored::*;
        print!("{}\t", symlink.red());
    }


    pub fn output(dirs: Vec<DirEntry>) {
        for item in dirs {
            if item.file_type().unwrap().is_dir() {
                Output::print_dir(item.file_name().into_string().unwrap());
            } else if item.file_type().unwrap().is_file() {
                Output::print_file(item.file_name().into_string().unwrap());
            } else {
                Output::print_symlink(item.file_name().into_string().unwrap());
            }
        }
        println!();
    }
}