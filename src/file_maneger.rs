use std::env::consts::OS;
use std::fs;
use std::path::Path;

pub struct Directory {
    pub path: String,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub extention: String,
    pub path: String,
    pub full_name: String,
}

impl Directory {
    fn new(path: &String) -> Directory {
        let path = path.clone();
        Directory { path }
    }

    fn get_list(dir: Directory) {
        let file_list = fs::read_dir(dir.path).unwrap();
    }
}

impl File {
    pub fn new(path: &str) -> File {
        let file_raw = Path::new(path);

        let name = String::from(file_raw.file_stem().unwrap().to_str().unwrap());
        let extention = String::from(file_raw.extension().unwrap().to_str().unwrap());
        let path = file_raw.parent().unwrap();

        let path = fs::canonicalize(path).unwrap();
        let path = String::from(path.to_str().unwrap());

        let full_name: String = match OS {
            "windows" => format!("{}\\{}.{}", &path, &name, &extention),
            _ => format!("{}/{}.{}", &path, &name, &extention),
        };

        File {
            name,
            extention,
            path,
            full_name,
        }
    }
    pub fn move_file(file: File, to_directory: Directory) -> std::io::Result<()> {
        let file_full_path = file.full_name;
        let file_name = format!("{}.{}", file.name, file.extention);
        let to_name = match OS {
            "windows" => format!("{}\\{}", to_directory.path, file_name),
            _ => format!("{}/{}", to_directory.path, file_name),
        };
        fs::rename(file_full_path, to_name)?;

        Ok(())
    }
}
