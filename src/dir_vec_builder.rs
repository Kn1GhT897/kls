use std::path::Path;
use std::fs::{read_dir, DirEntry};
pub struct DirVecBuilder;

impl DirVecBuilder {
    fn check_show_all(options: String) -> bool {
        match options.find("a") {
            Some(_) => true,
            None => false,
        }
    }


    pub fn build_vec(options: String, path: &Path) -> Vec<DirEntry> {
        let files = read_dir(path).unwrap();

        let mut result: Vec<DirEntry> = vec![];

        if !DirVecBuilder::check_show_all(options.clone()) {
            for item in files.filter(|x| {
                match x {
                    Ok(item) => item.file_name().into_string().unwrap().find(".") != Some(0 as usize),
                    _ => false,
                }
            }) {
                result.push(item.unwrap());
            }
        } else {
            for item in files.into_iter() {
                result.push(item.unwrap()); 
            }
        }

        result
    }
}