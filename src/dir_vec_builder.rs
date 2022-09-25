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


    pub fn build_vec(options: String, path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
        let files = match read_dir(path) {
            Ok(item) => item,
            Err(e) => return Err(e),
        };
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

        Ok(result)
    }

}