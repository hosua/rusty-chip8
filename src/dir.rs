pub struct Navigator;

use std::vec::Vec;

impl Navigator {
    pub fn select_game(path: &str) -> String {
        // let paths = std::fs::read_dir(path).unwrap();    
        let mut path_vec: Vec<String> = Vec::new();
        {
            let mut i = 0;
            let paths = walkdir::WalkDir::new(path);    
            for p in paths {
                i += 1;
                let abs_path = p.unwrap();
                let file_name = abs_path.file_name().to_str().unwrap().to_string();
                let path_str = abs_path.path().to_str().unwrap().to_string();
                let meta = std::fs::metadata(path_str.clone()).unwrap();
                path_vec.push(path_str.clone());
                if meta.is_dir() {
                    println!("{}", path_str);
                } else {
                    println!("{}. {}", i, file_name);
                }
            }
        }
        
        loop {
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).ok();
            let trimmed = user_input.trim(); 
            match trimmed.parse::<u32>() {
                Ok(i) => {
                    if i <= 0 || i > path_vec.len() as u32 { 
                        println!("{} is outside the boundaries of the listed files.", i);
                    } else {
                        let sel_str = path_vec[i as usize - 1].to_string();
                        println!("Selected: {}) {}", i, sel_str);
                        return sel_str;
                    }
                }
                Err(..) => { 
                    eprintln!("Input needs to be a valid number") 
                }
            }
        }
    }
}

