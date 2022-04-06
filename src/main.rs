use std::path::Path;
use std::result::Result;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
// #[derive(Debug, Clone)]
// struct NotPathError;

fn find_by_extension(directory_path: &str, extention: &str) -> Option<Box<dyn Error>> {
    let dir_path = Path::new(directory_path);
    let dir_iter_result = dir_path.read_dir();
    if dir_iter_result.is_ok() == true {
        let dir_iter = dir_iter_result.unwrap();
            // let mut res_vec: Vec<String> = Vec::new();
        for item in dir_iter {
            if let Ok(dir_entry) = item {
                let loc_path = dir_entry.path();
                if loc_path.is_dir() {
                    if let Some(str_path) = loc_path.to_str() {
                        find_by_extension(str_path, extention);
                    }
                } else if loc_path.is_file() {
                    let str_file_extension = loc_path.extension()?.to_str()?;
                    // if let Some(file_extension) =  loc_path.extension() {
                    //     if let Some(str_file_extension) = file_extension.to_str() {
                    if str_file_extension == extention {
                        println!("{:?}", loc_path);
                        let file_pre = File::open(loc_path);
                        if file_pre.is_ok() {
                            let file = BufReader::new(file_pre.unwrap());
                            let mut cnt  = file.lines().count();
                            println!("{:?}", cnt);
                        }
                        
                    }
                    //     }
                    // }
                }
            }
        }
    } else {
        return Some(Box::new(dir_iter_result.unwrap_err()));
    }
    
    Option::None
}

fn main() {
    if let Some(error) = find_by_extension(r"C:\Users\User\find_by_extention", "rs") {
        println!("{:?}", error);
    }
}